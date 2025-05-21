use std::f64::consts::PI;

#[allow(dead_code)]
pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[allow(dead_code)]
pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
}
