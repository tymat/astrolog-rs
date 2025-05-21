use crate::calc::utils::{degrees_to_radians, normalize_angle, radians_to_degrees};

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

pub fn calculate_obliquity(t: f64) -> f64 {
    // Calculate mean obliquity of the ecliptic
    23.43929111 - 0.013004167 * t - 0.0000001639 * t * t + 0.0000005036 * t * t * t
}

pub fn calculate_sidereal_time(t: f64, longitude: f64) -> f64 {
    // Calculate mean sidereal time at Greenwich
    let mst =
        280.46061837 + 360.98564736629 * (t * 36525.0) + t * t * (0.000387933 - t / 38710000.0);

    // Add longitude and normalize
    normalize_angle(mst + longitude)
}
