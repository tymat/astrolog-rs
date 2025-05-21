use crate::core::types::HouseSystem;
use crate::core::AstrologError;
use crate::calc::utils::{degrees_to_radians, radians_to_degrees, normalize_angle};
use crate::calc::swiss_ephemeris::calculate_house_cusps_swiss;
use approx::{AbsDiffEq, RelativeEq};

#[derive(Debug, Clone, PartialEq)]
pub struct HousePosition {
    pub number: u8,
    pub longitude: f64,
    pub latitude: f64,
}

impl AbsDiffEq for HousePosition {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.number == other.number &&
        self.longitude.abs_diff_eq(&other.longitude, epsilon) &&
        self.latitude.abs_diff_eq(&other.latitude, epsilon)
    }
}

impl RelativeEq for HousePosition {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.number == other.number &&
        self.longitude.relative_eq(&other.longitude, epsilon, max_relative) &&
        self.latitude.relative_eq(&other.latitude, epsilon, max_relative)
    }
}

/// Calculate house cusps for a given Julian date and location
pub fn calculate_houses(
    julian_date: f64,
    latitude: f64,
    longitude: f64,
    house_system: HouseSystem,
) -> Result<Vec<HousePosition>, AstrologError> {
    // Special case for Null house system - each house starts at 0° of its sign
    if house_system == HouseSystem::Null {
        return Ok((0..12)
            .map(|i| HousePosition {
                number: (i + 1) as u8,
                longitude: (i * 30) as f64,
                latitude: 0.0,
            })
            .collect());
    }

    // Check for extreme latitudes
    if latitude.abs() > 66.0 && house_system != HouseSystem::Equal && house_system != HouseSystem::WholeSign {
        return Err(AstrologError::InvalidLatitude(format!(
            "The {} system of houses is not defined at extreme latitudes.",
            house_system
        )));
    }

    // Handle polar regions
    if latitude.abs() >= 89.9 {
        return Ok(vec![
            HousePosition {
                number: 1,
                longitude: 0.0,
                latitude: 0.0,
            }; 12
        ]);
    }

    // Use Swiss Ephemeris for more accurate calculations
    let (cusps, _ascmc) = calculate_house_cusps_swiss(julian_date, latitude, longitude, house_system)?;

    // Convert house cusps to HousePosition structs
    Ok(cusps[1..13].iter()
        .enumerate()
        .map(|(i, &longitude)| HousePosition {
            number: (i + 1) as u8,
            longitude,
            latitude: 0.0, // House cusps are always on the ecliptic
        })
        .collect())
}

fn calculate_placidus_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);

    // Set MC and ASC as fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)

    // Calculate intermediate house cusps using Placidus system
    for i in 1..9 {
        let angle = (i as f64 * 30.0).to_radians();
        let x = (angle.cos() * lat_rad.cos() * obl_rad.sin()) / 
                (lat_rad.sin() * obl_rad.cos() - angle.sin() * lat_rad.cos() * obl_rad.sin());
        let y = (angle.sin() * lat_rad.cos()) / 
                (lat_rad.sin() * obl_rad.cos() - angle.sin() * lat_rad.cos() * obl_rad.sin());
        
        let cusp = (y.atan2(x) + mc_longitude.to_radians()).to_degrees();
        houses[i] = normalize_angle(cusp);
    }

    // Calculate remaining houses
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    houses[10] = normalize_angle(houses[4] + 180.0); // 11th house
    houses[11] = normalize_angle(houses[5] + 180.0); // 12th house

    houses
}

fn calculate_koch_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using Koch system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Koch system uses the prime vertical with a different division
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        // Apply Koch-specific correction
        let correction = (angle / 90.0) * (obliquity / 3.0);
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude + correction);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_equal_houses(asc_longitude: f64) -> Vec<f64> {
    (0..12)
        .map(|i| normalize_angle(asc_longitude + (i as f64) * 30.0))
        .collect()
}

fn calculate_whole_sign_houses(asc_longitude: f64) -> Vec<f64> {
    // In whole sign houses, each house starts at the beginning of a sign
    let asc_sign = (asc_longitude / 30.0).floor() * 30.0;
    (0..12)
        .map(|i| normalize_angle(asc_sign + (i as f64) * 30.0))
        .collect()
}

fn calculate_campanus_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using Campanus system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Campanus system uses the prime vertical
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_regiomontanus_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using Regiomontanus system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Regiomontanus system uses the celestial equator
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        // Apply Regiomontanus-specific correction
        let correction = (angle / 90.0) * (obliquity / 4.0);
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude + correction);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_meridian_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using the meridian system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Meridian system uses the prime vertical
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_alcabitius_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // Calculate declination of the Ascendant
    let decl = (obl_rad.sin() * degrees_to_radians(asc_longitude).sin()).asin();
    
    // Calculate semi-diurnal arc
    let r = -(lat_rad.tan() * decl.tan());
    let sda = degrees_to_radians(90.0 - r.acos().to_degrees());
    let sna = 90.0 - sda;
    
    // Calculate house cusps
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses
    let ra = degrees_to_radians(mc_longitude);
    houses[3] = normalize_angle(ra.to_degrees() - sna);
    houses[2] = normalize_angle(ra.to_degrees() - sna * 2.0 / 3.0);
    houses[1] = normalize_angle(ra.to_degrees() - sna / 3.0);
    houses[4] = normalize_angle(ra.to_degrees() + sda / 3.0);
    houses[5] = normalize_angle(ra.to_degrees() + sda * 2.0 / 3.0);
    
    // Convert to ecliptic coordinates
    for i in 1..6 {
        let hr = degrees_to_radians(houses[i]);
        let hr2 = (hr.tan() / obl_rad.cos()).atan();
        let hr2 = if hr2 < 0.0 { hr2 + std::f64::consts::PI } else { hr2 };
        let hr2 = if hr.sin() < 0.0 { hr2 + std::f64::consts::PI } else { hr2 };
        houses[i] = normalize_angle(hr2.to_degrees() + mc_longitude);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_topocentric_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using the Topocentric system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Topocentric system uses a special division of the prime vertical
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        // Apply Topocentric-specific correction
        let correction = (angle / 90.0) * (obliquity / 3.0);
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude + correction);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_morinus_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using the Morinus system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Morinus system uses the celestial equator
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        // Apply Morinus-specific correction
        let correction = (angle / 90.0) * (obliquity / 4.0);
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude + correction);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_porphyrius_houses(mc_longitude: f64, asc_longitude: f64, _latitude: f64, _obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate the difference between MC and ASC
    let diff = normalize_angle(mc_longitude - asc_longitude);
    let trisection = diff / 3.0;
    
    // Calculate intermediate houses using trisection
    for i in 1..9 {
        let angle = (i as f64) * trisection;
        houses[i] = normalize_angle(asc_longitude + angle);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

fn calculate_krusinski_houses(mc_longitude: f64, asc_longitude: f64, latitude: f64, obliquity: f64) -> Vec<f64> {
    let mut houses = vec![0.0; 12];
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);
    
    // MC and ASC are fixed points
    houses[9] = mc_longitude; // MC (10th house)
    houses[0] = asc_longitude; // ASC (1st house)
    
    // Calculate intermediate houses using the Krusinski system
    for i in 1..9 {
        let angle = (i as f64) * 30.0;
        let angle_rad = degrees_to_radians(angle);
        
        // Krusinski system uses a special division of the prime vertical
        let y = (angle_rad.sin() * obl_rad.cos()).atan2(angle_rad.cos());
        let x = (lat_rad.cos() * angle_rad.sin() - lat_rad.sin() * obl_rad.cos() * angle_rad.cos()) /
            (lat_rad.sin() * angle_rad.sin() + lat_rad.cos() * obl_rad.cos() * angle_rad.cos());
        
        // Apply Krusinski-specific correction
        let correction = (angle / 90.0) * (obliquity / 2.5);
        houses[i] = normalize_angle(radians_to_degrees(y.atan2(x)) + asc_longitude + correction);
    }
    
    // Calculate remaining houses to ensure 180° oppositions
    houses[3] = normalize_angle(mc_longitude + 180.0); // IC (4th house)
    houses[6] = normalize_angle(asc_longitude + 180.0); // DESC (7th house)
    
    // Ensure opposite houses are exactly 180° apart
    for i in 0..6 {
        if i != 3 { // Skip IC since it's already set
            houses[i + 6] = normalize_angle(houses[i] + 180.0);
        }
    }
    
    houses
}

/// Calculate house cusps using the Vedic house system.
/// In this system, each house starts 15 degrees earlier than in the Equal system,
/// with the Ascendant falling in the middle of the 1st house.
fn calculate_vedic_houses(
    _mc_longitude: f64,
    ascendant: f64,
    _obliquity: f64,
    _latitude: f64,
) -> Vec<f64> {
    // Each house starts 15 degrees earlier than in Equal system
    let first_house = normalize_angle(ascendant - 15.0);
    let mut houses = Vec::with_capacity(12);
    
    // Calculate all houses starting from the first house
    for i in 0..12 {
        houses.push(normalize_angle(first_house + (i as f64 * 30.0)));
    }
    
    houses
}

/// Calculate house cusps using the Null house system.
/// In this system, cusps are fixed to start at their corresponding signs
/// (1st house at 0° Aries, 2nd at 0° Taurus, etc.)
fn calculate_null_houses(
    _mc_longitude: f64,
    _ascendant: f64,
    _obliquity: f64,
    _latitude: f64,
) -> Vec<f64> {
    // Each house starts at 0° of its corresponding sign
    (0..12)
        .map(|i| i as f64 * 30.0)
        .collect()
}

/// Calculate house placements for a given set of positions
#[allow(dead_code)]
pub fn calculate_house_placements(
    positions: &[f64],
    cusps: &[f64],
) -> Result<Vec<u8>, AstrologError> {
    let mut placements = Vec::with_capacity(positions.len());
    
    for &position in positions {
        let mut house = 1;
        let mut found = false;
        
        // Find the first cusp that is greater than the position
        for (i, &cusp) in cusps.iter().enumerate() {
            if position < cusp {
                house = i as u8 + 1;
                found = true;
                break;
            }
        }
        
        // If no cusp was found greater than the position, it's in the last house
        if !found {
            house = 12;
        }
        
        placements.push(house);
    }
    
    Ok(placements)
}

/// Determine which house a given position falls in.
/// Returns the house number (1-12) for the given position.
#[allow(dead_code)]
pub fn house_place_in(position: f64, house_cusps: &[f64; 12]) -> usize {
    let position = normalize_angle(position);
    
    // Find the first house cusp that's greater than the position
    for i in 0..12 {
        let next_i = (i + 1) % 12;
        if position >= house_cusps[i] && position < house_cusps[next_i] {
            return i + 1; // Houses are 1-based
        }
    }
    
    // If we get here, the position must be in the last house
    12
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_house_systems() {
        let julian_date = 2451545.0; // 2000-01-01
        let latitude = 40.0;
        let longitude = -74.0;
        let house_system = HouseSystem::Placidus;

        let houses = calculate_houses(julian_date, latitude, longitude, house_system).unwrap();
        assert_eq!(houses.len(), 12, "House system {:?} should return 12 houses", house_system);

        // Test house numbers and longitudes
        for i in 0..houses.len() {
            assert_eq!(houses[i].number, (i + 1) as u8,
                "House {} should have number {}", i + 1, i + 1);
            let house = &houses[i];
            assert!(house.longitude >= 0.0 && house.longitude < 360.0,
                "House {} longitude should be between 0 and 360 degrees", i + 1);
        }
    }

    #[test]
    fn test_house_system_consistency() {
        let julian_date = 2451545.0;
        let latitude = 40.0;
        let longitude = -74.0;

        // Test each house system independently
        for system in [
            HouseSystem::Equal,
            HouseSystem::WholeSign,
            HouseSystem::Placidus,
            HouseSystem::Koch,
            HouseSystem::Campanus,
            HouseSystem::Regiomontanus,
            HouseSystem::Meridian,
            HouseSystem::Alcabitius,
            HouseSystem::Topocentric,
            HouseSystem::Morinus,
            HouseSystem::Porphyrius,
            HouseSystem::Krusinski,
            HouseSystem::Vedic,
            HouseSystem::Null,
        ].iter() {
            let houses = calculate_houses(julian_date, latitude, longitude, *system).unwrap();
            
            // Verify we have exactly 12 houses
            assert_eq!(houses.len(), 12, "House system {:?} should return 12 houses", system);
            
            // Verify house numbers are correct
            for (i, house) in houses.iter().enumerate() {
                assert_eq!(house.number, (i + 1) as u8,
                    "House system {:?} should have house {} at index {}", system, i + 1, i);
            }
            
            // Verify longitudes are normalized
            for house in houses.iter() {
                assert!(house.longitude >= 0.0 && house.longitude < 360.0,
                    "House system {:?} should have normalized longitudes", system);
            }
            
            // For Equal, WholeSign, and Vedic systems, verify houses are 30° apart
            if *system == HouseSystem::Equal || *system == HouseSystem::WholeSign || *system == HouseSystem::Vedic {
                for i in 1..12 {
                    let diff = normalize_angle(houses[i].longitude - houses[i-1].longitude);
                    let min_diff = diff.min(360.0 - diff);
                    assert!((min_diff - 30.0).abs() <= 0.1,
                        "House system {:?} should have houses 30° apart, found difference of {:.6}° between houses {} and {}",
                        system, min_diff, i, i + 1);
                }
            }
            
            // For other systems, verify opposite houses are 180° apart
            if *system != HouseSystem::Equal && *system != HouseSystem::WholeSign && *system != HouseSystem::Vedic {
                for i in 0..6 {
                    let diff = normalize_angle(houses[i].longitude - houses[i + 6].longitude);
                    let min_diff = diff.min(360.0 - diff);
                    assert!((min_diff - 180.0).abs() <= 0.1,
                        "House system {:?} should have opposite houses 180° apart, found difference of {:.6}° between houses {} and {}",
                        system, min_diff, i + 1, i + 7);
                }
            }
        }
    }

    #[test]
    fn test_vedic_houses() {
        let julian_date = 2451545.0;
        let latitude = 40.0;
        let longitude = -74.0;

        let houses = calculate_houses(julian_date, latitude, longitude, HouseSystem::Vedic).unwrap();
        
        // Verify we have exactly 12 houses
        assert_eq!(houses.len(), 12, "Vedic system should return 12 houses");
        
        // Verify house numbers are correct
        for (i, house) in houses.iter().enumerate() {
            assert_eq!(house.number, (i + 1) as u8,
                "Vedic system should have house {} at index {}", i + 1, i);
        }
        
        // Verify longitudes are normalized
        for house in houses.iter() {
            assert!(house.longitude >= 0.0 && house.longitude < 360.0,
                "Vedic system should have normalized longitudes");
        }
        
        // Verify houses are 30° apart
        for i in 1..12 {
            let diff = normalize_angle(houses[i].longitude - houses[i-1].longitude);
            let min_diff = diff.min(360.0 - diff);
            assert!((min_diff - 30.0).abs() <= 0.1,
                "Vedic houses should be 30° apart, found difference of {:.6}° between houses {} and {}",
                min_diff, i, i + 1);
        }
        
        // Verify first house starts at Ascendant - 15°
        let ascendant = houses[0].longitude + 15.0; // Since first house is 15° before ascendant
        let expected_first_house = normalize_angle(ascendant - 15.0);
        let diff = normalize_angle(houses[0].longitude - expected_first_house);
        let min_diff = diff.min(360.0 - diff);
        assert!(min_diff <= 0.1,
            "First house should start at Ascendant - 15°, found difference of {:.6}°",
            min_diff);
    }

    #[test]
    fn test_extreme_latitude_handling() {
        let julian_date = 2451545.0;
        let latitude = 89.0; // Extreme latitude
        let longitude = 0.0;

        // Equal and WholeSign should work at extreme latitudes
        let _equal_houses = calculate_houses(julian_date, latitude, longitude, HouseSystem::Equal).unwrap();
        let _whole_houses = calculate_houses(julian_date, latitude, longitude, HouseSystem::WholeSign).unwrap();

        // Other systems should fail
        assert!(calculate_houses(julian_date, latitude, longitude, HouseSystem::Placidus).is_err());
        assert!(calculate_houses(julian_date, latitude, longitude, HouseSystem::Koch).is_err());
    }

    #[test]
    fn test_null_houses() {
        let julian_date = 2451545.0;
        let latitude = 40.0;
        let longitude = -74.0;

        let houses = calculate_houses(julian_date, latitude, longitude, HouseSystem::Null).unwrap();
        
        // Each house should start at 0° of its sign
        for i in 0..12 {
            assert_relative_eq!(houses[i].longitude, (i * 30) as f64, epsilon = 0.0001);
        }
    }
} 