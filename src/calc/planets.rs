use crate::calc::swiss_ephemeris::{self, map_planet_to_swe};
use crate::calc::utils::{degrees_to_radians, radians_to_degrees};
use crate::calc::vsop87;
use crate::core::types::AstrologError;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};
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
    pub longitude: f64,      // Longitude in degrees
    pub latitude: f64,       // Latitude in degrees
    pub speed: f64,          // Daily motion in degrees
    pub is_retrograde: bool, // Whether the planet is retrograde
    pub house: Option<u8>,   // House number (1-12) if applicable
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
#[allow(dead_code)]
pub fn calculate_planet_positions(jd: f64) -> Result<Vec<PlanetPosition>, AstrologError> {
    let mut positions = Vec::with_capacity(10);

    // Convert Julian date to DateTime
    let jd_epoch = 2440587.5; // Unix epoch in Julian days
    let unix_seconds = ((jd - jd_epoch) * 86400.0) as i64;
    let naive = NaiveDateTime::from_timestamp_opt(unix_seconds, 0).ok_or_else(|| {
        AstrologError::CalculationError {
            message: "Invalid date".to_string(),
        }
    })?;
    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive);

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
    ]
    .iter()
    {
        match calculate_planet_position(
            *planet,
            datetime.year(),
            datetime.month() as i32,
            datetime.day() as i32,
            datetime.hour() as f64
                + datetime.minute() as f64 / 60.0
                + datetime.second() as f64 / 3600.0,
        ) {
            Ok(position) => positions.push(position),
            Err(e) => return Err(AstrologError::CalculationError { message: e }),
        }
    }

    Ok(positions)
}

/// Calculate the position of a planet for a given date and time
pub fn calculate_planet_position(
    planet: Planet,
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
) -> Result<PlanetPosition, String> {
    // Convert date and time to Julian date using Swiss Ephemeris
    let swe_planet = map_planet_to_swe(planet).ok_or_else(|| "Invalid planet".to_string())?;

    // Calculate position using Swiss Ephemeris
    let (longitude, latitude, _distance) =
        swiss_ephemeris::calculate_planet_position_swiss(swe_planet, year, month, day, hour)
            .map_err(|e| e.to_string())?;

    // Calculate speed by getting positions slightly before and after
    let dt = 0.01; // 0.01 days = 14.4 minutes
    let hour_before = hour - dt * 24.0;
    let hour_after = hour + dt * 24.0;

    let (long_before, _, _) =
        swiss_ephemeris::calculate_planet_position_swiss(swe_planet, year, month, day, hour_before)
            .map_err(|e| e.to_string())?;

    let (long_after, _, _) =
        swiss_ephemeris::calculate_planet_position_swiss(swe_planet, year, month, day, hour_after)
            .map_err(|e| e.to_string())?;

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

    // Normalize speed to [-180, 180] range
    speed = speed.rem_euclid(360.0);
    if speed > 180.0 {
        speed -= 360.0;
    }

    Ok(PlanetPosition::new(longitude, latitude, speed, speed < 0.0))
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
    let (earth_long, _earth_lat, _earth_z) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);
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
    let longitude = normalize_longitude(
        mean_longitude
            + 6.289 * (mean_anomaly * PI / 180.0).sin()
            + 1.274 * ((2.0 * mean_longitude - mean_anomaly) * PI / 180.0).sin()
            + 0.658 * (2.0 * mean_longitude * PI / 180.0).sin()
            + 0.214 * (2.0 * mean_anomaly * PI / 180.0).sin()
            + 0.114 * (2.0 * ascending_node * PI / 180.0).sin(),
    );

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
    let (mercury_long, mercury_lat, mercury_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        mercury_long,
        mercury_lat,
        mercury_r,
        earth_long,
        earth_lat,
        earth_r,
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
    let (venus_long, venus_lat, venus_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        venus_long, venus_lat, venus_r, earth_long, earth_lat, earth_r,
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
    let (mars_long, mars_lat, mars_r) = vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        mars_long, mars_lat, mars_r, earth_long, earth_lat, earth_r,
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
    let (jupiter_long, jupiter_lat, jupiter_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        jupiter_long,
        jupiter_lat,
        jupiter_r,
        earth_long,
        earth_lat,
        earth_r,
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
    let (saturn_long, saturn_lat, saturn_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        saturn_long,
        saturn_lat,
        saturn_r,
        earth_long,
        earth_lat,
        earth_r,
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
    let (uranus_long, uranus_lat, uranus_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        uranus_long,
        uranus_lat,
        uranus_r,
        earth_long,
        earth_lat,
        earth_r,
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
    let (neptune_long, neptune_lat, neptune_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        neptune_long,
        neptune_lat,
        neptune_r,
        earth_long,
        earth_lat,
        earth_r,
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
    let (pluto_long, pluto_lat, pluto_r) =
        vsop87::heliocentric_coordinates(t, a, e, i, l, lp, node);

    // Calculate Earth's position
    let a_earth = 1.00000261;
    let e_earth = 0.01671123 - 0.00004392 * t;
    let i_earth = -0.00001531 - 0.01294668 * t;
    let l_earth = 100.46457166 + 35999.37244981 * t;
    let lp_earth = 102.93768193 + 0.32327364 * t;
    let node_earth = 0.0;
    let (earth_long, earth_lat, earth_r) = vsop87::heliocentric_coordinates(
        t, a_earth, e_earth, i_earth, l_earth, lp_earth, node_earth,
    );

    // Convert to geocentric coordinates
    let (longitude, latitude) = vsop87::heliocentric_to_geocentric(
        pluto_long, pluto_lat, pluto_r, earth_long, earth_lat, earth_r,
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
    let (earth_long, earth_lat, earth_r) =
        vsop87::heliocentric_coordinates(t, a_e, e_e, i_e, l_e, lp_e, node_e);
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
        .map(|(curr, prev)| (curr.speed < 0.0) != (prev.speed < 0.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::swiss_ephemeris;
    use approx::assert_relative_eq;

    // Natal chart data: October 24, 1977, 04:56 AM, 121:03:03E 14:38:55N
    const TEST_YEAR: i32 = 1977;
    const TEST_MONTH: i32 = 10;
    const TEST_DAY: i32 = 24;
    const TEST_HOUR: f64 = 4.0 + 56.0 / 60.0; // 04:56 AM

    fn setup() -> Result<(), String> {
        // Initialize Swiss Ephemeris before running tests
        swiss_ephemeris::init_swiss_ephemeris()
            .map_err(|e| format!("Failed to initialize Swiss Ephemeris: {}", e))
    }

    // Helper function to normalize angles to [-180, 180] range
    fn normalize_angle(angle: f64) -> f64 {
        let mut normalized = angle % 360.0;
        if normalized > 180.0 {
            normalized -= 360.0;
        } else if normalized < -180.0 {
            normalized += 360.0;
        }
        normalized
    }

    // Helper function to print position details
    fn print_position_details(planet: &str, expected: f64, actual: f64) {
        println!("{} position test failed:", planet);
        println!("  Expected: {:.3}°", expected);
        println!("  Actual:   {:.3}°", actual);
        println!("  Difference: {:.3}°", (actual - expected).abs());
    }

    #[test]
    fn test_sun_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Sun, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Sun position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 210.674, epsilon = 1e-3) {
            print_position_details("Sun", 210.674, position.longitude);
            assert_relative_eq!(position.longitude, 210.674, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 0.0, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_moon_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Moon, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Moon position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 358.594, epsilon = 1e-3) {
            print_position_details("Moon", 358.594, position.longitude);
            assert_relative_eq!(position.longitude, 358.594, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 1.518, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_mercury_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Mercury, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Mercury position: {}", e))?;
        if !approx::relative_eq!(position.longitude, 214.148, epsilon = 1e-3) {
            print_position_details("Mercury", 214.148, position.longitude);
            assert_relative_eq!(position.longitude, 214.148, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 0.234, epsilon = 0.01);
        Ok(())
    }

    #[test]
    fn test_venus_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Venus, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Venus position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 188.853, epsilon = 1e-3) {
            print_position_details("Venus", 188.853, position.longitude);
            assert_relative_eq!(position.longitude, 188.853, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 1.563, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_mars_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Mars, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Mars position: {}", e))?;
        if !approx::relative_eq!(position.longitude, 118.878, epsilon = 1e-3) {
            print_position_details("Mars", 118.878, position.longitude);
            assert_relative_eq!(position.longitude, 118.878, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 1.184, epsilon = 0.05);
        Ok(())
    }

    #[test]
    fn test_jupiter_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Jupiter, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Jupiter position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 96.142, epsilon = 1e-3) {
            print_position_details("Jupiter", 96.142, position.longitude);
            assert_relative_eq!(position.longitude, 96.142, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, -0.352, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_saturn_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Saturn, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Saturn position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 148.485, epsilon = 1e-3) {
            print_position_details("Saturn", 148.485, position.longitude);
            assert_relative_eq!(position.longitude, 148.485, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 1.169, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_uranus_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Uranus, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Uranus position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 221.400, epsilon = 1e-3) {
            print_position_details("Uranus", 221.400, position.longitude);
            assert_relative_eq!(position.longitude, 221.400, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 0.390, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_neptune_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Neptune, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Neptune position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 254.296, epsilon = 1e-3) {
            print_position_details("Neptune", 254.296, position.longitude);
            assert_relative_eq!(position.longitude, 254.296, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 1.432, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_pluto_position() -> Result<(), String> {
        setup()?;
        let position =
            calculate_planet_position(Planet::Pluto, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Pluto position: {}", e))?;
        // Expected values from Swiss Ephemeris (geocentric)
        if !approx::relative_eq!(position.longitude, 194.736, epsilon = 1e-3) {
            print_position_details("Pluto", 194.736, position.longitude);
            assert_relative_eq!(position.longitude, 194.736, epsilon = 1e-3);
        }
        assert_relative_eq!(position.latitude, 16.545, epsilon = 1e-2);
        Ok(())
    }

    #[test]
    fn test_planet_positions_consistency() -> Result<(), String> {
        setup()?;
        // Test that positions are consistent across multiple calculations
        let pos1 =
            calculate_planet_position(Planet::Sun, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate first Sun position: {}", e))?;
        let pos2 =
            calculate_planet_position(Planet::Sun, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate second Sun position: {}", e))?;
        assert_relative_eq!(pos1.longitude, pos2.longitude, epsilon = 1e-10);
        assert_relative_eq!(pos1.latitude, pos2.latitude, epsilon = 1e-10);
        Ok(())
    }

    // #[test]
    // fn test_retrograde_motion() -> Result<(), String> {
    //     setup()?;
    //     // Test for retrograde motion detection
    //     let pos1 = calculate_planet_position(Planet::Mars, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
    //         .map_err(|e| format!("Failed to calculate first Mars position: {}", e))?;
    //     let pos2 = calculate_planet_position(Planet::Mars, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR + 1.0)
    //         .map_err(|e| format!("Failed to calculate second Mars position: {}", e))?;
    //     let pos3 = calculate_planet_position(Planet::Mars, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR + 2.0)
    //         .map_err(|e| format!("Failed to calculate third Mars position: {}", e))?;
    //
    //     // Check if the planet is moving backwards (retrograde)
    //     let motion1 = normalize_angle(pos2.longitude - pos1.longitude);
    //     let motion2 = normalize_angle(pos3.longitude - pos2.longitude);
    //
    //     println!("Mars motion test:");
    //     println!("  Position 1: {:.3}°", pos1.longitude);
    //     println!("  Position 2: {:.3}°", pos2.longitude);
    //     println!("  Position 3: {:.3}°", pos3.longitude);
    //     println!("  Motion 1: {:.3}°", motion1);
    //     println!("  Motion 2: {:.3}°", motion2);
    //
    //     // If both motions are negative, the planet is retrograde
    //     assert!(motion1 < 0.0 || motion2 < 0.0, "Expected retrograde motion not detected");
    //     Ok(())
    // }

    // #[test]
    // fn test_stationary_points() -> Result<(), String> {
    //     setup()?;
    //     // Test for stationary points (where a planet changes direction)
    //     let pos1 = calculate_planet_position(Planet::Mercury, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
    //         .map_err(|e| format!("Failed to calculate first Mercury position: {}", e))?;
    //     let pos2 = calculate_planet_position(Planet::Mercury, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR + 1.0)
    //         .map_err(|e| format!("Failed to calculate second Mercury position: {}", e))?;
    //     let pos3 = calculate_planet_position(Planet::Mercury, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR + 2.0)
    //         .map_err(|e| format!("Failed to calculate third Mercury position: {}", e))?;
    //
    //     // Calculate the motion between points
    //     let motion1 = normalize_angle(pos2.longitude - pos1.longitude);
    //     let motion2 = normalize_angle(pos3.longitude - pos2.longitude);
    //
    //     println!("Mercury stationary point test:");
    //     println!("  Position 1: {:.3}°", pos1.longitude);
    //     println!("  Position 2: {:.3}°", pos2.longitude);
    //     println!("  Position 3: {:.3}°", pos3.longitude);
    //     println!("  Motion 1: {:.3}°", motion1);
    //     println!("  Motion 2: {:.3}°", motion2);
    //
    //     // If the motions have different signs, we've found a stationary point
    //     assert!(
    //         (motion1 > 0.0 && motion2 < 0.0) || (motion1 < 0.0 && motion2 > 0.0),
    //         "Expected stationary point not detected"
    //     );
    //     Ok(())
    // }
}
