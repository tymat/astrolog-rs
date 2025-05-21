use crate::core::types::AstrologError;
use serde::{Serialize, Deserialize};
use crate::calc::vsop87;
use crate::calc::utils::{radians_to_degrees, degrees_to_radians};
use crate::calc::swiss_ephemeris::{self, map_planet_to_swe};
use std::f64::consts::PI;
use chrono::{DateTime, NaiveDateTime, Utc, Datelike, Timelike};

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
    MeanNode,
    TrueNode,
    MeanLilith,
    TrueLilith,
    Chiron,
    Ceres,
    Pallas,
    Juno,
    Vesta,
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

/// Normalize longitude to 0-360 degrees
fn normalize_longitude(longitude: f64) -> f64 {
    let mut normalized = longitude % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
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
        match calculate_planet_position(*planet, jd) {
            Ok(position) => positions.push(position),
            Err(e) => return Err(AstrologError::CalculationError { message: e }),
        }
    }
    
    Ok(positions)
}

/// Calculate the position of a planet for a given date and time
pub fn calculate_planet_position(
    planet: Planet,
    julian_date: f64,
) -> Result<PlanetPosition, String> {
    // Convert Julian date to DateTime
    let jd_epoch = 2440587.5; // Unix epoch in Julian days
    let unix_seconds = ((julian_date - jd_epoch) * 86400.0) as i64;
    let naive = NaiveDateTime::from_timestamp_opt(unix_seconds, 0)
        .ok_or_else(|| "Invalid date".to_string())?;
    let datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);

    // Get Swiss Ephemeris planet enum
    let swe_planet = map_planet_to_swe(planet).ok_or_else(|| "Invalid planet".to_string())?;

    // Calculate position using Swiss Ephemeris
    let (longitude, latitude, _distance) = swiss_ephemeris::calculate_planet_position_swiss(
        swe_planet,
        datetime.year(),
        datetime.month() as i32,
        datetime.day() as i32,
        datetime.hour() as f64 + datetime.minute() as f64 / 60.0 + datetime.second() as f64 / 3600.0,
    ).map_err(|e| e.to_string())?;

    // Calculate speed by getting positions slightly before and after
    let dt = 0.01; // 0.01 days = 14.4 minutes
    let jd_before = julian_date - dt;
    let jd_after = julian_date + dt;

    let unix_seconds_before = ((jd_before - jd_epoch) * 86400.0) as i64;
    let naive_before = NaiveDateTime::from_timestamp_opt(unix_seconds_before, 0)
        .ok_or_else(|| "Invalid date".to_string())?;
    let datetime_before: DateTime<Utc> = DateTime::<Utc>::from_utc(naive_before, Utc);

    let unix_seconds_after = ((jd_after - jd_epoch) * 86400.0) as i64;
    let naive_after = NaiveDateTime::from_timestamp_opt(unix_seconds_after, 0)
        .ok_or_else(|| "Invalid date".to_string())?;
    let datetime_after: DateTime<Utc> = DateTime::<Utc>::from_utc(naive_after, Utc);

    let (long_before, _, _) = swiss_ephemeris::calculate_planet_position_swiss(
        swe_planet,
        datetime_before.year(),
        datetime_before.month() as i32,
        datetime_before.day() as i32,
        datetime_before.hour() as f64 + datetime_before.minute() as f64 / 60.0 + datetime_before.second() as f64 / 3600.0,
    ).map_err(|e| e.to_string())?;

    let (long_after, _, _) = swiss_ephemeris::calculate_planet_position_swiss(
        swe_planet,
        datetime_after.year(),
        datetime_after.month() as i32,
        datetime_after.day() as i32,
        datetime_after.hour() as f64 + datetime_after.minute() as f64 / 60.0 + datetime_after.second() as f64 / 3600.0,
    ).map_err(|e| e.to_string())?;

    // Calculate speed using central difference
    let mut speed = (long_after - long_before) / (2.0 * dt);
    
    // Handle crossing the 0°/360° boundary
    if (long_after - long_before).abs() > 180.0 {
        if long_after > long_before {
            speed = (long_after - long_before - 360.0) / (2.0 * dt);
        } else {
            speed = (long_after - long_before + 360.0) / (2.0 * dt);
        }
    }

    Ok(PlanetPosition::new(
        longitude,
        latitude,
        speed,
        speed < 0.0,
    ))
}

/// Calculate Sun's position
#[allow(dead_code)]
fn calculate_sun_position(t: f64) -> Result<PlanetPosition, String> {
    // Earth orbital elements (Meeus Table 31.A)
    let a = 1.00000261; // AU
    let e = 0.01671123 - 0.00004392 * t;
    let i = -0.00001531 - 0.01294668 * t;
    let l = 100.46457166 + 35999.37244981 * t;
    let lp = 102.93768193 + 0.32327364 * t;
    let node = 0.0;
    let (earth_long, _earth_lat, _earth_z) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    let longitude = (earth_long + 180.0).rem_euclid(360.0);
    Ok(PlanetPosition::new(longitude, 0.0, 0.0, false))
}

/// Calculate Moon's position
#[allow(dead_code)]
fn calculate_moon_position(t: f64) -> Result<PlanetPosition, String> {
    // Simplified lunar model
    let mean_longitude = 218.316 + 13.176396 * t;
    let mean_anomaly = 134.963 + 13.064993 * t;
    let ascending_node = 125.045 - 0.052992 * t;
    
    // Calculate longitude with correction terms
    let longitude = normalize_longitude(mean_longitude + 
        6.289 * (mean_anomaly * PI / 180.0).sin() +
        1.274 * ((2.0 * mean_longitude - mean_anomaly) * PI / 180.0).sin() +
        0.658 * (2.0 * mean_longitude * PI / 180.0).sin() +
        0.214 * (2.0 * mean_anomaly * PI / 180.0).sin() +
        0.114 * (2.0 * ascending_node * PI / 180.0).sin());
    
    // Calculate latitude using orbital inclination
    let inclination = 5.145;
    let latitude = inclination * (longitude - ascending_node).sin();
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Mercury's position
#[allow(dead_code)]
fn calculate_mercury_position(t: f64) -> Result<PlanetPosition, String> {
    // Mercury orbital elements (Meeus Table 31.A)
    let a = 0.38709843; // AU
    let e = 0.20563661 + 0.00002123 * t;
    let i = 7.00497902 - 0.00594749 * t;
    let l = 252.25032350 + 149472.67411175 * t;
    let lp = 77.45779628 + 0.15940013 * t;
    let node = 48.33076593 - 0.12534081 * t;
    
    // Calculate heliocentric coordinates
    let (mercury_long, mercury_lat, mercury_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        mercury_long, mercury_lat, mercury_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Venus's position
#[allow(dead_code)]
fn calculate_venus_position(t: f64) -> Result<PlanetPosition, String> {
    // Venus orbital elements (Meeus Table 31.A)
    let a = 0.72332102; // AU
    let e = 0.00676399 - 0.00005107 * t;
    let i = 3.39777545 - 0.00043494 * t;
    let l = 181.97909950 + 58517.81538729 * t;
    let lp = 131.60246718 + 0.00268329 * t;
    let node = 76.67984255 - 0.27769418 * t;
    
    // Calculate heliocentric coordinates
    let (venus_long, venus_lat, venus_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        venus_long, venus_lat, venus_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Mars's position
#[allow(dead_code)]
fn calculate_mars_position(t: f64) -> Result<PlanetPosition, String> {
    // Mars orbital elements (Meeus Table 31.A)
    let a = 1.52371243; // AU
    let e = 0.09336511 + 0.00009149 * t;
    let i = 1.85181869 - 0.00724757 * t;
    let l = 355.45332620 + 19140.30268499 * t;
    let lp = 336.04084219 + 0.44390164 * t;
    let node = 49.71355184 - 0.29257343 * t;
    
    // Calculate heliocentric coordinates
    let (mars_long, mars_lat, mars_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        mars_long, mars_lat, mars_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Jupiter's position
#[allow(dead_code)]
fn calculate_jupiter_position(t: f64) -> Result<PlanetPosition, String> {
    // Jupiter orbital elements (Meeus Table 31.A)
    let a = 5.20248019; // AU
    let e = 0.04853590 + 0.00018026 * t;
    let i = 1.29861416 - 0.00322699 * t;
    let l = 34.33479152 + 3034.90371757 * t;
    let lp = 14.72847983 + 0.21252668 * t;
    let node = 100.29282654 + 0.13032614 * t;
    
    // Calculate heliocentric coordinates
    let (jupiter_long, jupiter_lat, jupiter_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        jupiter_long, jupiter_lat, jupiter_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Saturn's position
#[allow(dead_code)]
fn calculate_saturn_position(t: f64) -> Result<PlanetPosition, String> {
    // Saturn orbital elements (Meeus Table 31.A)
    let a = 9.54149883; // AU
    let e = 0.05550825 - 0.00034664 * t;
    let i = 2.49424102 + 0.00451969 * t;
    let l = 49.55953891 + 1222.11379404 * t;
    let lp = 92.86136063 + 0.54179478 * t;
    let node = 113.63998702 - 0.25015002 * t;
    
    // Calculate heliocentric coordinates
    let (saturn_long, saturn_lat, saturn_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        saturn_long, saturn_lat, saturn_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Uranus's position
#[allow(dead_code)]
fn calculate_uranus_position(t: f64) -> Result<PlanetPosition, String> {
    // Uranus orbital elements (Meeus Table 31.A)
    let a = 19.18797948; // AU
    let e = 0.04731826 + 0.00000745 * t;
    let i = 0.77298127 - 0.00180155 * t;
    let l = 313.23810451 + 428.48202785 * t;
    let lp = 172.43404441 + 0.09266985 * t;
    let node = 74.22992501 + 0.04240589 * t;
    
    // Calculate heliocentric coordinates
    let (uranus_long, uranus_lat, uranus_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        uranus_long, uranus_lat, uranus_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Neptune's position
#[allow(dead_code)]
fn calculate_neptune_position(t: f64) -> Result<PlanetPosition, String> {
    // Neptune orbital elements (Meeus Table 31.A)
    let a = 30.06952752; // AU
    let e = 0.00860648 + 0.00000215 * t;
    let i = 1.77005520 + 0.00022400 * t;
    let l = 304.88003403 + 218.45945325 * t;
    let lp = 46.68158724 + 0.01009938 * t;
    let node = 131.78635853 - 0.00606302 * t;
    
    // Calculate heliocentric coordinates
    let (neptune_long, neptune_lat, neptune_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        neptune_long, neptune_lat, neptune_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

/// Calculate Pluto's position
#[allow(dead_code)]
fn calculate_pluto_position(t: f64) -> Result<PlanetPosition, String> {
    // Pluto orbital elements (Meeus Table 31.A)
    let a = 39.48686035; // AU
    let e = 0.24885238 + 0.00006016 * t;
    let i = 17.14104260 + 0.00000501 * t;
    let l = 238.96535011 + 145.18042903 * t;
    let lp = 224.09702598 - 0.00968827 * t;
    let node = 110.30167986 - 0.00809981 * t;
    
    // Calculate heliocentric coordinates
    let (pluto_long, pluto_lat, pluto_r) = vsop87::heliocentric_coordinates(
        t, a, e, i, l, lp, node
    );
    
    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth
    );
    
    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        pluto_long, pluto_lat, pluto_r,
        earth_long, earth_lat, earth_r
    );
    
    Ok(PlanetPosition::new(longitude, latitude, 0.0, false))
}

#[allow(dead_code)]
fn calculate_geocentric_planet_position(
    t: f64,
    a: f64,
    e: f64,
    i: f64,
    l: f64,
    lp: f64,
    node: f64,
) -> PlanetPosition {
    // Get heliocentric coordinates for planet
    let (pl_long, pl_lat, pl_r) = vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);
    let pl_long_rad = degrees_to_radians(pl_long);
    let pl_lat_rad = degrees_to_radians(pl_lat);
    // Rectangular coordinates for planet
    let x_p = pl_r * pl_long_rad.cos() * pl_lat_rad.cos();
    let y_p = pl_r * pl_long_rad.sin() * pl_lat_rad.cos();
    let z_p = pl_r * pl_lat_rad.sin();
    // Earth orbital elements (Meeus Table 31.A)
    let a_e = 1.00000261; // AU
    let e_e = 0.01671123 - 0.00004392 * t;
    let i_e = -0.00001531 - 0.01294668 * t;
    let l_e = 100.46457166 + 35999.37244981 * t;
    let lp_e = 102.93768193 + 0.32327364 * t;
    let node_e = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_e, e_e, i_e, l_e, lp_e, node_e
    );
    let earth_long_rad = degrees_to_radians(earth_long);
    let earth_lat_rad = degrees_to_radians(earth_lat);
    let x_e = earth_r * earth_long_rad.cos() * earth_lat_rad.cos();
    let y_e = earth_r * earth_long_rad.sin() * earth_lat_rad.cos();
    let z_e = earth_r * earth_lat_rad.sin();
    // Geocentric rectangular coordinates (Planet - Earth)
    let x = x_p - x_e;
    let y = y_p - y_e;
    let z = z_p - z_e;
    // Convert to ecliptic longitude and latitude
    let _r = (x * x + y * y + z * z).sqrt();
    let longitude = radians_to_degrees(y.atan2(x)).rem_euclid(360.0);
    let latitude = radians_to_degrees(z.atan2((x * x + y * y).sqrt()));
    PlanetPosition::new(longitude, latitude, 0.0, false)
}

/// Calculate planetary aspects for a given set of positions
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn calculate_retrogrades(positions: &[PlanetPosition]) -> Vec<bool> {
    positions.iter().map(|p| p.speed < 0.0).collect()
}

/// Calculate planetary stations
#[allow(dead_code)]
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

    const TEST_JD: f64 = 2443439.5; // October 24, 1977

    #[test]
    fn test_sun_position() {
        let position = calculate_sun_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 209.784, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.0, epsilon = 1e-3);
    }

    #[test]
    fn test_moon_position() {
        let position = calculate_moon_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 218.944, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, -1.817, epsilon = 1e-3);
    }

    #[test]
    fn test_mercury_position() {
        let position = calculate_mercury_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 212.492, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.366, epsilon = 1e-3);
    }

    #[test]
    fn test_venus_position() {
        let position = calculate_venus_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 187.671, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 1.563, epsilon = 1e-3);
    }

    #[test]
    fn test_mars_position() {
        let position = calculate_mars_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 118.665, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 1.184, epsilon = 1e-3);
    }

    #[test]
    fn test_jupiter_position() {
        let position = calculate_jupiter_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 96.334, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, -0.352, epsilon = 1e-3);
    }

    #[test]
    fn test_saturn_position() {
        let position = calculate_saturn_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 148.556, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 1.169, epsilon = 1e-3);
    }

    #[test]
    fn test_uranus_position() {
        let position = calculate_uranus_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 221.573, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 0.390, epsilon = 1e-3);
    }

    #[test]
    fn test_neptune_position() {
        let position = calculate_neptune_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 254.602, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 1.432, epsilon = 1e-3);
    }

    #[test]
    fn test_pluto_position() {
        let position = calculate_pluto_position(vsop87::julian_centuries(TEST_JD)).unwrap();
        assert_relative_eq!(position.longitude, 195.072, epsilon = 1e-3);
        assert_relative_eq!(position.latitude, 16.545, epsilon = 1e-3);
    }

    #[test]
    fn test_planet_positions_consistency() {
        // Only test the first 10 planets (Sun through Pluto)
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
        // Test Mercury retrograde (January 14, 2024 - February 3, 2024)
        let jd_mercury_retrograde = 2460314.5; // January 14, 2024 - start of retrograde
        let position = calculate_planet_position(Planet::Mercury, jd_mercury_retrograde).unwrap();
        
        // Calculate positions for a few days before and after to verify retrograde
        let jd_before = jd_mercury_retrograde - 1.0; // Increased time interval to 1 day
        let jd_after = jd_mercury_retrograde + 1.0;  // Increased time interval to 1 day
        let pos_before = calculate_planet_position(Planet::Mercury, jd_before).unwrap();
        let pos_after = calculate_planet_position(Planet::Mercury, jd_after).unwrap();
        
        println!("Mercury: before: lon={:.6} speed={:.6}, on: lon={:.6} speed={:.6}, after: lon={:.6} speed={:.6}",
            pos_before.longitude, pos_before.speed,
            position.longitude, position.speed,
            pos_after.longitude, pos_after.speed);
        
        // Check if the planet is moving backwards (retrograde)
        assert!(position.speed < 0.0, "Mercury should be retrograde on January 14, 2024");

        // Test Mars retrograde (October 30, 2022 - January 12, 2023)
        let jd_mars_retrograde = 2459890.5; // October 30, 2022 - start of retrograde
        let position = calculate_planet_position(Planet::Mars, jd_mars_retrograde).unwrap();
        
        // Calculate positions for a few days before and after to verify retrograde
        let jd_before = jd_mars_retrograde - 1.0; // Increased time interval to 1 day
        let jd_after = jd_mars_retrograde + 1.0;  // Increased time interval to 1 day
        let pos_before = calculate_planet_position(Planet::Mars, jd_before).unwrap();
        let pos_after = calculate_planet_position(Planet::Mars, jd_after).unwrap();
        
        println!("Mars: before: lon={:.6} speed={:.6}, on: lon={:.6} speed={:.6}, after: lon={:.6} speed={:.6}",
            pos_before.longitude, pos_before.speed,
            position.longitude, position.speed,
            pos_after.longitude, pos_after.speed);
        
        // Check if the planet is moving backwards (retrograde)
        assert!(position.speed < 0.0, "Mars should be retrograde on October 30, 2022");

        // Test Jupiter direct motion (not retrograde during this period)
        let jd_jupiter_direct = 2460314.5; // January 14, 2024
        let position = calculate_planet_position(Planet::Jupiter, jd_jupiter_direct).unwrap();
        
        // Calculate positions for a few days before and after to verify direct motion
        let jd_before = jd_jupiter_direct - 1.0; // Increased time interval to 1 day
        let jd_after = jd_jupiter_direct + 1.0;  // Increased time interval to 1 day
        let pos_before = calculate_planet_position(Planet::Jupiter, jd_before).unwrap();
        let pos_after = calculate_planet_position(Planet::Jupiter, jd_after).unwrap();
        
        println!("Jupiter: before: lon={:.6} speed={:.6}, on: lon={:.6} speed={:.6}, after: lon={:.6} speed={:.6}",
            pos_before.longitude, pos_before.speed,
            position.longitude, position.speed,
            pos_after.longitude, pos_after.speed);
        
        // Check if the planet is moving forwards (direct)
        assert!(position.speed > 0.0, "Jupiter should be in direct motion on January 14, 2024");
    }

    #[test]
    fn test_stationary_points() {
        // Test Mercury stationary (January 14, 2024 - start of retrograde)
        let jd_mercury_stationary = 2460314.5; // January 14, 2024
        let position = calculate_planet_position(Planet::Mercury, jd_mercury_stationary).unwrap();
        
        // Calculate positions for a few days before and after to verify stationary point
        let jd_before = jd_mercury_stationary - 1.0; // Increased time interval to 1 day
        let jd_after = jd_mercury_stationary + 1.0;  // Increased time interval to 1 day
        let pos_before = calculate_planet_position(Planet::Mercury, jd_before).unwrap();
        let pos_after = calculate_planet_position(Planet::Mercury, jd_after).unwrap();
        
        println!("Mercury Stationary: before: lon={:.6} speed={:.6}, on: lon={:.6} speed={:.6}, after: lon={:.6} speed={:.6}",
            pos_before.longitude, pos_before.speed,
            position.longitude, position.speed,
            pos_after.longitude, pos_after.speed);
        
        // Check if the planet is changing direction (stationary)
        let direction_before = pos_before.speed > 0.0;
        let direction_after = pos_after.speed < 0.0;
        assert_ne!(direction_before, direction_after, "Mercury should be stationary on January 14, 2024");
    }
} 