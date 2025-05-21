use crate::calc::utils::{degrees_to_radians, normalize_angle, radians_to_degrees};

/// Calculates the Ascendant (rising sign) and Midheaven (MC) angles for a given time and location.
///
/// The Ascendant is the point where the ecliptic intersects the eastern horizon,
/// representing the sign that was rising at the time of birth.
/// The Midheaven is the point where the ecliptic intersects the meridian,
/// representing the highest point in the chart.
///
/// # Arguments
///
/// * `sidereal_time` - The local sidereal time in hours (0-24)
/// * `latitude` - The geographical latitude in degrees (-90 to 90)
/// * `obliquity` - The obliquity of the ecliptic in degrees
///
/// # Returns
///
/// A tuple containing:
/// * The Ascendant angle in degrees (0-360)
/// * The Midheaven angle in degrees (0-360)
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::angles::calculate_angles;
///
/// let sidereal_time = 12.0; // Noon
/// let latitude = 40.0;      // New York
/// let obliquity = 23.4367;  // Current obliquity
///
/// let (ascendant, midheaven) = calculate_angles(sidereal_time, latitude, obliquity);
/// println!("Ascendant: {}°, Midheaven: {}°", ascendant, midheaven);
/// ```
#[allow(dead_code)]
pub fn calculate_angles(sidereal_time: f64, latitude: f64, obliquity: f64) -> (f64, f64) {
    let st_rad = degrees_to_radians(sidereal_time);
    let lat_rad = degrees_to_radians(latitude);
    let obl_rad = degrees_to_radians(obliquity);

    // Calculate MC (Midheaven)
    let mc_longitude = normalize_angle(sidereal_time);

    // Calculate ASC (Ascendant)
    let y = (st_rad.sin() * obl_rad.cos()).atan2(st_rad.cos());
    let x = (lat_rad.cos() * st_rad.sin() - lat_rad.sin() * obl_rad.cos() * st_rad.cos())
        / (lat_rad.sin() * st_rad.sin() + lat_rad.cos() * obl_rad.cos() * st_rad.cos());
    let asc_longitude = normalize_angle(radians_to_degrees(y.atan2(x)));

    (mc_longitude, asc_longitude)
}

/// Calculates the obliquity of the ecliptic for a given Julian date.
///
/// The obliquity of the ecliptic is the angle between the plane of the Earth's
/// orbit and the plane of the Earth's equator. This angle changes over time
/// due to the precession of the equinoxes.
///
/// # Arguments
///
/// * `t` - The Julian centuries since J2000.0
///
/// # Returns
///
/// The obliquity of the ecliptic in degrees
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::angles::calculate_obliquity;
///
/// let t = 0.0; // J2000.0
/// let obliquity = calculate_obliquity(t);
/// println!("Obliquity at J2000.0: {}°", obliquity);
/// ```
#[allow(dead_code)]
pub fn calculate_obliquity(t: f64) -> f64 {
    // Calculate mean obliquity of the ecliptic
    23.43929111 - 0.013004167 * t - 0.0000001639 * t * t + 0.0000005036 * t * t * t
}

/// Calculates the local sidereal time for a given time and location.
///
/// Sidereal time is a timekeeping system that measures the Earth's rotation
/// relative to the fixed stars rather than the Sun. It's used to determine
/// the positions of celestial objects in the sky.
///
/// # Arguments
///
/// * `t` - The Julian centuries since J2000.0
/// * `longitude` - The geographical longitude in degrees (-180 to 180)
///
/// # Returns
///
/// The local sidereal time in hours (0-24)
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::angles::calculate_sidereal_time;
///
/// let t = 0.0;        // J2000.0
/// let longitude = -74.0; // New York
///
/// let lst = calculate_sidereal_time(t, longitude);
/// println!("Local Sidereal Time: {} hours", lst);
/// ```
#[allow(dead_code)]
pub fn calculate_sidereal_time(t: f64, longitude: f64) -> f64 {
    // Calculate mean sidereal time at Greenwich
    let mst =
        280.46061837 + 360.98564736629 * (t * 36525.0) + t * t * (0.000387933 - t / 38710000.0);

    // Add longitude and normalize
    normalize_angle(mst + longitude)
}
