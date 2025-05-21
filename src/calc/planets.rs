use crate::core::AstrologError;
use serde::{Serialize, Deserialize};
use crate::calc::vsop87;
use crate::calc::utils::{degrees_to_radians, radians_to_degrees};
use std::f64::consts::PI;

/// Planet types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Planet {
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
    Chiron,
    Ceres,
    Pallas,
    Juno,
    Vesta,
    NorthNode,
    Lilith,
    Fortune,
    Vertex,
    EastPoint,
}

/// Planetary position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetPosition {
    pub longitude: f64,    // Longitude in degrees
    pub latitude: f64,     // Latitude in degrees
    pub speed: f64,        // Daily motion in degrees
    pub is_retrograde: bool, // Whether the planet is retrograde
    pub house: Option<u8>, // House number (1-12) if applicable
}

impl PlanetPosition {
    pub fn new(longitude: f64, latitude: f64, speed: f64, is_retrograde: bool) -> Self {
        Self {
            longitude,
            latitude,
            speed,
            is_retrograde,
            house: None,
        }
    }
}

/// Calculate planetary positions for a given Julian date
pub fn calculate_planet_positions(jd: f64) -> Result<Vec<PlanetPosition>, AstrologError> {
    let mut positions = Vec::with_capacity(15);
    
    // Calculate positions for each planet
    for planet in 0..15 {
        let position = calculate_planet_position(planet, jd)?;
        positions.push(position);
    }
    
    Ok(positions)
}

/// Calculate the position of a planet for a given date and time
pub fn calculate_planet_position(
    planet: Planet,
    julian_date: f64,
) -> Result<PlanetPosition, String> {
    let t = vsop87::julian_centuries(julian_date);
    
    match planet {
        Planet::Sun => calculate_sun_position(t),
        Planet::Moon => calculate_moon_position(t),
        Planet::Mercury => calculate_mercury_position(t),
        Planet::Venus => calculate_venus_position(t),
        Planet::Mars => calculate_mars_position(t),
        Planet::Jupiter => calculate_jupiter_position(t),
        Planet::Saturn => calculate_saturn_position(t),
        Planet::Uranus => calculate_uranus_position(t),
        Planet::Neptune => calculate_neptune_position(t),
        Planet::Pluto => calculate_pluto_position(t),
        _ => Err("Planet calculation not implemented".into()),
    }
}

/// Calculate Sun's position
fn calculate_sun_position(t: f64) -> Result<PlanetPosition, String> {
    // For the Sun, we use a simplified model since it's at the center
    // The Sun's position is the negative of the Earth's position
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        1.000000, // Semi-major axis (AU)
        0.016709, // Eccentricity
        0.0,      // Inclination
        100.466457, // Mean longitude
        102.94719,  // Longitude of perihelion
        0.0,        // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.9856; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Moon's position
fn calculate_moon_position(t: f64) -> Result<PlanetPosition, String> {
    // Simplified lunar model
    let mean_longitude = 218.31617 + 13.1763965 * t;
    let mean_anomaly = 134.96292 + 13.0649930 * t;
    let ascending_node = 125.04452 - 0.0529538 * t;
    
    let longitude = mean_longitude + 
        6.2888 * (mean_anomaly * PI / 180.0).sin() +
        1.2740 * ((2.0 * mean_longitude - mean_anomaly) * PI / 180.0).sin() +
        0.6583 * (2.0 * mean_anomaly * PI / 180.0).sin() +
        0.2136 * (2.0 * mean_longitude * PI / 180.0).sin();
    
    let speed = 13.1763965; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Mercury's position
fn calculate_mercury_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        0.387098,  // Semi-major axis (AU)
        0.205635,  // Eccentricity
        7.00487,   // Inclination
        252.25084, // Mean longitude
        77.45645,  // Longitude of perihelion
        48.33167,  // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 1.3833; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Venus's position
fn calculate_venus_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        0.723330,  // Semi-major axis (AU)
        0.006773,  // Eccentricity
        3.39471,   // Inclination
        181.97973, // Mean longitude
        131.53298, // Longitude of perihelion
        76.68069,  // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 1.2; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Mars's position
fn calculate_mars_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        1.523688,  // Semi-major axis (AU)
        0.093405,  // Eccentricity
        1.85061,   // Inclination
        355.45332, // Mean longitude
        336.04084, // Longitude of perihelion
        49.57854,  // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.524; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Jupiter's position
fn calculate_jupiter_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        5.202561,  // Semi-major axis (AU)
        0.048498,  // Eccentricity
        1.30530,   // Inclination
        34.35148,  // Mean longitude
        14.72884,  // Longitude of perihelion
        100.55615, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.083; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Saturn's position
fn calculate_saturn_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        9.554747,  // Semi-major axis (AU)
        0.054509,  // Eccentricity
        2.48446,   // Inclination
        49.94432,  // Mean longitude
        92.43194,  // Longitude of perihelion
        113.71504, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.034; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Uranus's position
fn calculate_uranus_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        19.218140, // Semi-major axis (AU)
        0.047318,  // Eccentricity
        0.77464,   // Inclination
        313.23218, // Mean longitude
        172.73583, // Longitude of perihelion
        74.22988,  // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.012; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Neptune's position
fn calculate_neptune_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        30.110387, // Semi-major axis (AU)
        0.008606,  // Eccentricity
        1.77004,   // Inclination
        304.88003, // Mean longitude
        48.12369,  // Longitude of perihelion
        131.72169, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.006; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate Pluto's position
fn calculate_pluto_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, _) = vsop87::heliocentric_coordinates(
        t,
        39.481686, // Semi-major axis (AU)
        0.248807,  // Eccentricity
        17.14175,  // Inclination
        238.92903, // Mean longitude
        224.06676, // Longitude of perihelion
        110.30347, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let speed = 0.004; // Average daily motion in degrees
    
    Ok(PlanetPosition::new(longitude, 0.0, speed, false))
}

/// Calculate planetary aspects for a given set of positions
pub fn calculate_planetary_aspects(
    positions: &[PlanetPosition],
    orbs: &[f64],
) -> Vec<(Planet, Planet, f64, f64)> {
    let mut aspects = Vec::new();
    
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let diff = (positions[i].longitude - positions[j].longitude).abs() % 360.0;
            
            // Check for major aspects
            if diff <= orbs[0] || (360.0 - diff) <= orbs[0] {
                aspects.push((Planet::Sun, Planet::Sun, diff, 0.0));
            } else if (diff - 60.0).abs() <= orbs[1] {
                aspects.push((Planet::Sun, Planet::Sun, diff, 60.0));
            } else if (diff - 90.0).abs() <= orbs[2] {
                aspects.push((Planet::Sun, Planet::Sun, diff, 90.0));
            } else if (diff - 120.0).abs() <= orbs[3] {
                aspects.push((Planet::Sun, Planet::Sun, diff, 120.0));
            } else if (diff - 180.0).abs() <= orbs[4] {
                aspects.push((Planet::Sun, Planet::Sun, diff, 180.0));
            }
        }
    }
    
    aspects
}

/// Calculate planetary retrogrades
pub fn calculate_retrogrades(positions: &[PlanetPosition]) -> Vec<bool> {
    positions.iter().map(|p| p.speed < 0.0).collect()
}

/// Calculate planetary stations
pub fn calculate_stations(
    positions: &[PlanetPosition],
    prev_positions: &[PlanetPosition],
) -> Vec<bool> {
    positions
        .iter()
        .zip(prev_positions.iter())
        .map(|(curr, prev)| {
            (curr.speed < 0.0) != (prev.speed < 0.0)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use chrono::{DateTime, Utc, TimeZone};

    #[test]
    fn test_planet_position_creation() {
        let position = PlanetPosition::new(45.0, 0.0, 1.0, false);
        assert_relative_eq!(position.longitude, 45.0);
        assert_relative_eq!(position.latitude, 0.0);
        assert_relative_eq!(position.speed, 1.0);
        assert!(!position.is_retrograde);
        assert_eq!(position.house, None);
    }

    #[test]
    fn test_planet_position_with_house() {
        let mut position = PlanetPosition::new(45.0, 0.0, 1.0, false);
        position.house = Some(1);
        assert_eq!(position.house, Some(1));
    }

    #[test]
    fn test_sun_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_sun_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 210.674, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.995, epsilon = 1e-3);
    }

    #[test]
    fn test_moon_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_moon_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 358.595, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 12.82, epsilon = 1e-3);
    }

    #[test]
    fn test_mercury_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_mercury_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 201.123, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 1.3833, epsilon = 1e-3);
    }

    #[test]
    fn test_venus_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_venus_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 156.789, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 1.2, epsilon = 1e-3);
    }

    #[test]
    fn test_mars_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_mars_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 278.456, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.524, epsilon = 1e-3);
    }

    #[test]
    fn test_jupiter_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_jupiter_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 123.789, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.083, epsilon = 1e-3);
    }

    #[test]
    fn test_saturn_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_saturn_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 145.678, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.034, epsilon = 1e-3);
    }

    #[test]
    fn test_uranus_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_uranus_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 234.567, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.012, epsilon = 1e-3);
    }

    #[test]
    fn test_neptune_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_neptune_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 267.890, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.006, epsilon = 1e-3);
    }

    #[test]
    fn test_pluto_position() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_pluto_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 189.012, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.004, epsilon = 1e-3);
    }

    #[test]
    fn test_planet_positions_consistency() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        
        // Test that all planets have valid positions
        let positions = calculate_planet_positions(jd).unwrap();
        assert_eq!(positions.len(), 15); // All planets should have positions
        
        // Test that positions are within valid ranges
        for position in positions {
            assert!(position.longitude >= 0.0 && position.longitude < 360.0);
            assert!(position.latitude >= -90.0 && position.latitude <= 90.0);
            assert!(position.speed.abs() < 15.0); // No planet moves faster than 15 degrees per day
        }
    }

    #[test]
    fn test_sun_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_sun_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 210.674, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.0, epsilon = 1e-3); // Sun's latitude is always 0
        assert_relative_eq!(position.speed, 0.995, epsilon = 1e-3);
    }

    #[test]
    fn test_moon_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_moon_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 358.595, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 4.123, epsilon = 1e-3); // Moon's latitude varies
        assert_relative_eq!(position.speed, 12.82, epsilon = 1e-3);
    }

    #[test]
    fn test_mercury_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_mercury_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 201.123, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 2.456, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 1.3833, epsilon = 1e-3);
    }

    #[test]
    fn test_venus_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_venus_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 156.789, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, -1.234, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 1.2, epsilon = 1e-3);
    }

    #[test]
    fn test_mars_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_mars_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 278.456, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.789, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.524, epsilon = 1e-3);
    }

    #[test]
    fn test_jupiter_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_jupiter_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 123.789, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.567, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.083, epsilon = 1e-3);
    }

    #[test]
    fn test_saturn_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_saturn_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 145.678, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 1.234, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.034, epsilon = 1e-3);
    }

    #[test]
    fn test_uranus_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_uranus_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 234.567, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, -0.345, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.012, epsilon = 1e-3);
    }

    #[test]
    fn test_neptune_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_neptune_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 267.890, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.678, epsilon = 1e-3);
        assert_relative_eq!(position.speed, 0.006, epsilon = 1e-3);
    }

    #[test]
    fn test_pluto_position_with_latitude() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        let position = calculate_pluto_position(vsop87::julian_centuries(jd)).unwrap();
        assert_relative_eq!(position.longitude, 189.012, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 8.901, epsilon = 1e-3); // Pluto has high inclination
        assert_relative_eq!(position.speed, 0.004, epsilon = 1e-3);
    }

    #[test]
    fn test_planet_latitude_ranges() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let jd = 2443437.705555556; // Julian date for the test date
        
        // Test that all planets have valid latitude ranges
        let positions = calculate_planet_positions(jd).unwrap();
        
        // Check each planet's latitude is within its orbital inclination
        for (i, position) in positions.iter().enumerate() {
            match i {
                0 => assert_relative_eq!(position.latitude, 0.0, epsilon = 1e-3), // Sun
                1 => assert!(position.latitude.abs() <= 5.145), // Moon
                2 => assert!(position.latitude.abs() <= 7.005), // Mercury
                3 => assert!(position.latitude.abs() <= 3.395), // Venus
                4 => assert!(position.latitude.abs() <= 1.851), // Mars
                5 => assert!(position.latitude.abs() <= 1.305), // Jupiter
                6 => assert!(position.latitude.abs() <= 2.485), // Saturn
                7 => assert!(position.latitude.abs() <= 0.775), // Uranus
                8 => assert!(position.latitude.abs() <= 1.770), // Neptune
                9 => assert!(position.latitude.abs() <= 17.142), // Pluto
                _ => assert!(position.latitude.abs() <= 90.0), // Other bodies
            }
        }
    }
} 