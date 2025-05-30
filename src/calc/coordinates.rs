use crate::calc::utils::{degrees_to_radians, radians_to_degrees};
use crate::core::AstrologError;

/// Convert ecliptic coordinates to equatorial coordinates
#[allow(dead_code)]
pub fn ecliptic_to_equatorial(
    longitude: f64,
    latitude: f64,
    obliquity: f64,
) -> Result<(f64, f64), AstrologError> {
    // Normalize longitude to 0-360 range
    let mut _longitude = longitude % 360.0;
    if _longitude < 0.0 {
        _longitude += 360.0;
    }

    // Handle edge cases for latitude
    if latitude.abs() >= 90.0 {
        return Ok((_longitude, latitude.signum() * (90.0 - obliquity)));
    }

    // Convert angles to radians
    let lon_rad = degrees_to_radians(_longitude);
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);

    // Calculate right ascension
    let y = lon_rad.sin() * obl_rad.cos() - lat_rad.tan() * obl_rad.sin();
    let x = lon_rad.cos();
    let ra = y.atan2(x);

    // Calculate declination
    let dec =
        (lat_rad.sin() * obl_rad.cos() + lat_rad.cos() * obl_rad.sin() * lon_rad.sin()).asin();

    // Convert back to degrees
    let ra_deg = radians_to_degrees(ra);
    let dec_deg = radians_to_degrees(dec);

    // Normalize right ascension to 0-360 range
    let ra_normalized = if ra_deg < 0.0 { ra_deg + 360.0 } else { ra_deg };

    Ok((ra_normalized, dec_deg))
}

/// Convert equatorial coordinates to ecliptic coordinates
#[allow(dead_code)]
pub fn equatorial_to_ecliptic(
    right_ascension: f64,
    declination: f64,
    obliquity: f64,
) -> (f64, f64) {
    let ra_rad = right_ascension.to_radians();
    let dec_rad = declination.to_radians();
    let obl_rad = obliquity.to_radians();

    let sin_dec = dec_rad.sin();
    let cos_dec = dec_rad.cos();
    let sin_ra = ra_rad.sin();
    let cos_ra = ra_rad.cos();
    let sin_obl = obl_rad.sin();
    let cos_obl = obl_rad.cos();

    // Calculate latitude
    let lat = (sin_dec * cos_obl - cos_dec * sin_obl * sin_ra).asin();

    // Calculate longitude
    let lon = (cos_dec * cos_ra).atan2(cos_dec * sin_ra * cos_obl + sin_dec * sin_obl);

    (lon.to_degrees(), lat.to_degrees())
}

/// Convert equatorial coordinates to horizontal coordinates
#[allow(dead_code)]
pub fn equatorial_to_horizontal(
    ra: f64,
    dec: f64,
    _longitude: f64,
    latitude: f64,
    lst: f64,
) -> (f64, f64) {
    let ra_rad = ra.to_radians();
    let dec_rad = dec.to_radians();
    let lat_rad = latitude.to_radians();
    let lst_rad = lst.to_radians();

    let sin_dec = dec_rad.sin();
    let cos_dec = dec_rad.cos();
    let sin_lat = lat_rad.sin();
    let cos_lat = lat_rad.cos();

    // Calculate hour angle
    let ha = lst_rad - ra_rad;

    // Calculate altitude
    let alt = (sin_dec * sin_lat + cos_dec * cos_lat * ha.cos()).asin();

    // Calculate azimuth
    let az = (ha.sin()).atan2(ha.cos() * sin_lat - sin_dec / cos_dec * cos_lat);

    (az.to_degrees(), alt.to_degrees())
}

/// Calculate the sidereal time for a given Julian date and longitude
#[allow(dead_code)]
pub fn calculate_sidereal_time(julian_date: f64, longitude: f64) -> f64 {
    let t = (julian_date - 2451545.0) / 36525.0;

    // Calculate Greenwich Mean Sidereal Time
    let gmst = 280.46061837
        + 360.98564736629 * (julian_date - 2451545.0)
        + t * t * (0.000387933 - t / 38710000.0);

    // Add longitude to get Local Sidereal Time
    let lst = gmst + longitude;

    // Normalize to 0-360 degrees
    lst % 360.0
}

/// Calculate the Julian date for a given date and time
#[allow(dead_code)]
pub fn calculate_julian_date(
    year: i32,
    month: u32,
    day: u32,
    hour: f64,
    minute: f64,
    second: f64,
    timezone: f64,
) -> f64 {
    let mut y = year as f64;
    let mut m = month as f64;

    if m <= 2.0 {
        y -= 1.0;
        m += 12.0;
    }

    let a = (y / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    let jd =
        (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + day as f64 + b - 1524.5;

    // Add time of day
    let time = hour + minute / 60.0 + second / 3600.0 - timezone;

    jd + time / 24.0
}

#[allow(dead_code)]
pub fn normalize_coordinates(longitude: f64, latitude: f64) -> (f64, f64) {
    // Normalize longitude to 0-360 range
    let mut normalized_longitude = longitude % 360.0;
    if normalized_longitude < 0.0 {
        normalized_longitude += 360.0;
    }

    // Handle edge cases for latitude
    let normalized_latitude = if latitude.abs() >= 90.0 {
        if latitude > 0.0 {
            90.0
        } else {
            -90.0
        }
    } else {
        latitude
    };

    (normalized_longitude, normalized_latitude)
}

/// Convert spherical coordinates to rectangular coordinates.
///
/// # Arguments
/// * `r` - Radius (distance from origin)
/// * `azimuth` - Azimuth angle in radians
/// * `altitude` - Altitude angle in radians
/// * `x` - Output x coordinate
/// * `y` - Output y coordinate
/// * `z` - Output z coordinate
#[allow(dead_code)]
pub fn spherical_to_rectangular(
    r: f64,
    azimuth: f64,
    altitude: f64,
    x: &mut f64,
    y: &mut f64,
    z: &mut f64,
) {
    let cos_alt = altitude.cos();
    *x = r * cos_alt * azimuth.cos();
    *y = r * cos_alt * azimuth.sin();
    *z = r * altitude.sin();
}

/// Convert rectangular coordinates to spherical coordinates.
///
/// # Arguments
/// * `x` - X coordinate
/// * `y` - Y coordinate
/// * `z` - Z coordinate
/// * `r` - Output radius
/// * `azimuth` - Output azimuth angle in radians
/// * `altitude` - Output altitude angle in radians
#[allow(dead_code)]
pub fn rectangular_to_spherical(
    x: f64,
    y: f64,
    z: f64,
    r: &mut f64,
    azimuth: &mut f64,
    altitude: &mut f64,
) {
    *r = (x * x + y * y + z * z).sqrt();
    *azimuth = y.atan2(x);
    *altitude = (z / *r).asin();
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    const OBLIQUITY: f64 = 23.4367; // Current obliquity of the ecliptic

    #[test]
    fn test_ecliptic_to_equatorial_0_0() {
        let (ra, dec) = ecliptic_to_equatorial(0.0, 0.0, OBLIQUITY).unwrap();
        assert_relative_eq!(ra, 0.0, epsilon = 1e-10);
        assert_relative_eq!(dec, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_ecliptic_to_equatorial_90_0() {
        let (ra, dec) = ecliptic_to_equatorial(90.0, 0.0, OBLIQUITY).unwrap();
        assert_relative_eq!(ra, 90.0, epsilon = 1e-10);
        assert_relative_eq!(dec, OBLIQUITY, epsilon = 1e-10);
    }

    #[test]
    fn test_ecliptic_to_equatorial_180_0() {
        let (ra, dec) = ecliptic_to_equatorial(180.0, 0.0, OBLIQUITY).unwrap();
        assert_relative_eq!(ra, 180.0, epsilon = 1e-10);
        assert_relative_eq!(dec, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_ecliptic_to_equatorial_270_0() {
        let (ra, dec) = ecliptic_to_equatorial(270.0, 0.0, OBLIQUITY).unwrap();
        assert_relative_eq!(ra, 270.0, epsilon = 1e-10);
        assert_relative_eq!(dec, -OBLIQUITY, epsilon = 1e-10);
    }

    #[test]
    fn test_ecliptic_to_equatorial_with_latitude() {
        let (ra, dec) = ecliptic_to_equatorial(45.0, 30.0, OBLIQUITY).unwrap();
        // These values are calculated using standard astronomical coordinate transformation formulas
        assert_relative_eq!(ra, 30.6573524988265, epsilon = 1e-10);
        assert_relative_eq!(dec, 44.612822423799244, epsilon = 1e-10);
    }

    #[test]
    fn test_ecliptic_to_equatorial() {
        // Test North Pole
        let (_ra, dec) = ecliptic_to_equatorial(0.0, 90.0, OBLIQUITY).unwrap();
        assert_relative_eq!(dec, 90.0 - OBLIQUITY, epsilon = 1e-10);

        // Test South Pole
        let (_ra, dec) = ecliptic_to_equatorial(0.0, -90.0, OBLIQUITY).unwrap();
        assert_relative_eq!(dec, -90.0 + OBLIQUITY, epsilon = 1e-10);
    }

    #[test]
    fn test_spherical_rectangular_conversion() {
        let r = 1.0;
        let azimuth = PI / 4.0; // 45 degrees
        let altitude = PI / 6.0; // 30 degrees

        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        spherical_to_rectangular(r, azimuth, altitude, &mut x, &mut y, &mut z);

        // Convert back to spherical
        let mut r2 = 0.0;
        let mut azimuth2 = 0.0;
        let mut altitude2 = 0.0;

        rectangular_to_spherical(x, y, z, &mut r2, &mut azimuth2, &mut altitude2);

        // Check that we get back the original values
        assert_relative_eq!(r, r2, epsilon = 1e-10);
        assert_relative_eq!(azimuth, azimuth2, epsilon = 1e-10);
        assert_relative_eq!(altitude, altitude2, epsilon = 1e-10);

        // Check specific values
        assert_relative_eq!(x, 0.6123724356957945, epsilon = 1e-10);
        assert_relative_eq!(y, 0.6123724356957945, epsilon = 1e-10);
        assert_relative_eq!(z, 0.5, epsilon = 1e-10);
    }

    #[test]
    fn test_spherical_rectangular_edge_cases() {
        // Test zero radius
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        spherical_to_rectangular(0.0, PI / 4.0, PI / 6.0, &mut x, &mut y, &mut z);
        assert_relative_eq!(x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(z, 0.0, epsilon = 1e-10);

        // Test poles
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        spherical_to_rectangular(1.0, PI / 4.0, PI / 2.0, &mut x, &mut y, &mut z);
        assert_relative_eq!(x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(z, 1.0, epsilon = 1e-10);

        // Test negative radius
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        spherical_to_rectangular(-1.0, PI / 4.0, PI / 6.0, &mut x, &mut y, &mut z);
        assert_relative_eq!(x, -0.6123724356957945, epsilon = 1e-10);
        assert_relative_eq!(y, -0.6123724356957945, epsilon = 1e-10);
        assert_relative_eq!(z, -0.5, epsilon = 1e-10);
    }
}
