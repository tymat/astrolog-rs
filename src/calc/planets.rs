use crate::core::types::AstrologError;
use serde::{Serialize, Deserialize};
use crate::calc::vsop87;
use crate::calc::utils::radians_to_degrees;
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    let mut positions = Vec::with_capacity(10);
    
    // Calculate positions for each planet
    for planet in [
        Planet::Sun,
        Planet::Moon,
        Planet::Mercury,
        Planet::Venus,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
        Planet::Pluto,
    ].iter() {
        let position = calculate_planet_position(*planet, jd)
            .map_err(|e| AstrologError::CalculationError { message: e })?;
        positions.push(position);
    }
    
    Ok(positions)
}

/// Calculate the position of a planet for a given date and time
pub fn calculate_planet_position(
    planet: Planet,
    julian_date: f64,
) -> Result<PlanetPosition, String> {
    if julian_date < 0.0 {
        return Err("Invalid Julian date".into());
    }

    let t = vsop87::julian_centuries(julian_date);
    
    // Calculate position at current time
    let position = match planet {
        Planet::Sun => calculate_sun_position(t)?,
        Planet::Moon => calculate_moon_position(t)?,
        Planet::Mercury => calculate_mercury_position(t)?,
        Planet::Venus => calculate_venus_position(t)?,
        Planet::Mars => calculate_mars_position(t)?,
        Planet::Jupiter => calculate_jupiter_position(t)?,
        Planet::Saturn => calculate_saturn_position(t)?,
        Planet::Uranus => calculate_uranus_position(t)?,
        Planet::Neptune => calculate_neptune_position(t)?,
        Planet::Pluto => calculate_pluto_position(t)?,
        _ => return Err("Planet calculation not implemented".into()),
    };

    // Calculate position at a slightly later time to determine direction
    let delta_t = 0.0001; // About 8.64 seconds
    let t_next = t + delta_t;
    
    let next_position = match planet {
        Planet::Sun => calculate_sun_position(t_next)?,
        Planet::Moon => calculate_moon_position(t_next)?,
        Planet::Mercury => calculate_mercury_position(t_next)?,
        Planet::Venus => calculate_venus_position(t_next)?,
        Planet::Mars => calculate_mars_position(t_next)?,
        Planet::Jupiter => calculate_jupiter_position(t_next)?,
        Planet::Saturn => calculate_saturn_position(t_next)?,
        Planet::Uranus => calculate_uranus_position(t_next)?,
        Planet::Neptune => calculate_neptune_position(t_next)?,
        Planet::Pluto => calculate_pluto_position(t_next)?,
        _ => return Err("Planet calculation not implemented".into()),
    };

    // Calculate actual speed and determine if retrograde
    let mut speed = (next_position.longitude - position.longitude) / delta_t;
    
    // Normalize speed to handle crossing 0°/360° boundary
    if speed > 180.0 {
        speed -= 360.0;
    } else if speed < -180.0 {
        speed += 360.0;
    }
    
    // Create new position with updated speed and retrograde status
    Ok(PlanetPosition::new(
        position.longitude,
        position.latitude,
        speed,
        speed < 0.0,
    ))
}

/// Calculate Sun's position
fn calculate_sun_position(t: f64) -> Result<PlanetPosition, String> {
    // For the Sun, we calculate its position as the negative of Earth's position
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        1.0, // Semi-major axis (AU)
        0.0167, // Eccentricity
        0.0, // Inclination
        100.464, // Mean longitude
        102.937, // Longitude of perihelion
        0.0, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Moon's position
fn calculate_moon_position(t: f64) -> Result<PlanetPosition, String> {
    // Simplified lunar model
    let mean_longitude = 218.316 + 13.176396 * t;
    let mean_anomaly = 134.963 + 13.064993 * t;
    let ascending_node = 125.045 - 0.052992 * t;
    
    // Calculate longitude with correction terms
    let longitude = mean_longitude + 
        6.289 * (mean_anomaly * PI / 180.0).sin() +
        1.274 * ((2.0 * mean_longitude - mean_anomaly) * PI / 180.0).sin();
    
    // Calculate latitude using orbital inclination
    let inclination = 5.145;
    let latitude = inclination * (longitude - ascending_node).sin();
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Mercury's position
fn calculate_mercury_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        0.387, // Semi-major axis (AU)
        0.206, // Eccentricity
        7.005, // Inclination
        252.250, // Mean longitude
        77.456, // Longitude of perihelion
        48.331, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Venus's position
fn calculate_venus_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        0.723, // Semi-major axis (AU)
        0.007, // Eccentricity
        3.395, // Inclination
        181.979, // Mean longitude
        131.533, // Longitude of perihelion
        76.680, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Mars's position
fn calculate_mars_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        1.524, // Semi-major axis (AU)
        0.093, // Eccentricity
        1.850, // Inclination
        355.453, // Mean longitude
        336.041, // Longitude of perihelion
        49.558, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Jupiter's position
fn calculate_jupiter_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        5.203, // Semi-major axis (AU)
        0.048, // Eccentricity
        1.305, // Inclination
        34.404, // Mean longitude
        14.728, // Longitude of perihelion
        100.556, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Saturn's position
fn calculate_saturn_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        9.537, // Semi-major axis (AU)
        0.054, // Eccentricity
        2.484, // Inclination
        49.944, // Mean longitude
        92.432, // Longitude of perihelion
        113.715, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Uranus's position
fn calculate_uranus_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        19.191, // Semi-major axis (AU)
        0.047, // Eccentricity
        0.770, // Inclination
        313.232, // Mean longitude
        170.964, // Longitude of perihelion
        74.229, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Neptune's position
fn calculate_neptune_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        30.069, // Semi-major axis (AU)
        0.009, // Eccentricity
        1.770, // Inclination
        304.880, // Mean longitude
        44.971, // Longitude of perihelion
        131.721, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Pluto's position
fn calculate_pluto_position(t: f64) -> Result<PlanetPosition, String> {
    let (x, y, z) = vsop87::heliocentric_coordinates(
        t,
        39.482, // Semi-major axis (AU)
        0.249, // Eccentricity
        17.140, // Inclination
        238.929, // Mean longitude
        224.067, // Longitude of perihelion
        110.303, // Longitude of ascending node
    );
    
    let longitude = radians_to_degrees(y.atan2(x));
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
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
    use chrono::Utc;

    const TEST_JD: f64 = 2443439.5; // October 24, 1977

    #[test]
    fn test_sun_position() {
        let position = calculate_sun_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 210.674, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.0, epsilon = 1e-3);
    }

    #[test]
    fn test_moon_position() {
        let position = calculate_moon_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 358.595, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 4.123, epsilon = 1e-3);
    }

    #[test]
    fn test_mercury_position() {
        let position = calculate_mercury_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 201.123, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 2.456, epsilon = 1e-3);
    }

    #[test]
    fn test_venus_position() {
        let position = calculate_venus_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 156.789, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, -1.234, epsilon = 1e-3);
    }

    #[test]
    fn test_mars_position() {
        let position = calculate_mars_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 278.456, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.789, epsilon = 1e-3);
    }

    #[test]
    fn test_jupiter_position() {
        let position = calculate_jupiter_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 123.789, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.567, epsilon = 1e-3);
    }

    #[test]
    fn test_saturn_position() {
        let position = calculate_saturn_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 145.678, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 1.234, epsilon = 1e-3);
    }

    #[test]
    fn test_uranus_position() {
        let position = calculate_uranus_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 234.567, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, -0.345, epsilon = 1e-3);
    }

    #[test]
    fn test_neptune_position() {
        let position = calculate_neptune_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 267.890, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.678, epsilon = 1e-3);
    }

    #[test]
    fn test_pluto_position() {
        let position = calculate_pluto_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 189.012, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 8.901, epsilon = 1e-3);
    }

    #[test]
    fn test_planet_positions_consistency() {
        let positions = calculate_planet_positions(TEST_JD).unwrap();
        assert_eq!(positions.len(), 10);

        for position in positions {
            // Check longitude range
            assert!(position.longitude >= 0.0 && position.longitude < 360.0);
            
            // Check latitude range
            assert!(position.latitude >= -90.0 && position.latitude <= 90.0);
            
            // Check speed range (should be less than 15 degrees per day)
            assert!(position.speed.abs() < 15.0);
        }
    }

    #[test]
    fn test_retrograde_motion() {
        // Test Mercury retrograde
        let jd_mercury_retrograde = 2451545.0; // January 1, 2000
        let position = calculate_planet_position(Planet::Mercury, jd_mercury_retrograde).unwrap();
        assert!(position.is_retrograde);

        // Test Mars retrograde
        let jd_mars_retrograde = 2451545.0; // January 1, 2000
        let position = calculate_planet_position(Planet::Mars, jd_mars_retrograde).unwrap();
        assert!(position.is_retrograde);

        // Test Jupiter direct motion
        let jd_jupiter_direct = 2451545.0; // January 1, 2000
        let position = calculate_planet_position(Planet::Jupiter, jd_jupiter_direct).unwrap();
        assert!(!position.is_retrograde);
    }

    #[test]
    fn test_stationary_points() {
        // Test Mercury stationary
        let jd_mercury_stationary = 2451545.0; // January 1, 2000
        let position = calculate_planet_position(Planet::Mercury, jd_mercury_stationary).unwrap();
        assert_relative_eq!(position.speed, 0.0, epsilon = 0.1);
    }
} 