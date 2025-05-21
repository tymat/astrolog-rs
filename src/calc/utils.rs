use chrono::{DateTime, Utc};
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

#[allow(dead_code)]
pub fn date_to_julian(date: DateTime<Utc>) -> f64 {
    let unix_timestamp = date.timestamp() as f64;
    let julian_date = (unix_timestamp / 86400.0) + 2440587.5;
    julian_date
}

/// Calculate Julian centuries since J2000.0
#[allow(dead_code)]
pub fn julian_centuries(julian_date: f64) -> f64 {
    (julian_date - 2451545.0) / 36525.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_normalization() {
        let test_cases = [
            (0.0, 0.0),
            (360.0, 0.0),
            (720.0, 0.0),
            (180.0, 180.0),
            (540.0, 180.0),
            (-90.0, 270.0),
            (-360.0, 0.0),
            (-720.0, 0.0),
        ];

        for (input, expected) in test_cases.iter() {
            let result = normalize_angle(*input);
            assert!(
                (result - expected).abs() < 1e-10,
                "normalize_angle({}) = {}, expected {}",
                input,
                result,
                expected
            );
        }
    }

    #[test]
    fn test_degrees_to_radians() {
        let test_cases = [
            (0.0, 0.0),
            (90.0, PI / 2.0),
            (180.0, PI),
            (270.0, 3.0 * PI / 2.0),
            (360.0, 2.0 * PI),
        ];

        for (degrees, expected) in test_cases.iter() {
            let result = degrees_to_radians(*degrees);
            assert!(
                (result - expected).abs() < 1e-10,
                "degrees_to_radians({}) = {}, expected {}",
                degrees,
                result,
                expected
            );
        }
    }

    #[test]
    fn test_radians_to_degrees() {
        let test_cases = [
            (0.0, 0.0),
            (PI / 2.0, 90.0),
            (PI, 180.0),
            (3.0 * PI / 2.0, 270.0),
            (2.0 * PI, 360.0),
        ];

        for (radians, expected) in test_cases.iter() {
            let result = radians_to_degrees(*radians);
            assert!(
                (result - expected).abs() < 1e-10,
                "radians_to_degrees({}) = {}, expected {}",
                radians,
                result,
                expected
            );
        }
    }

    #[test]
    fn test_angle_conversion_roundtrip() {
        for degrees in 0..360 {
            let radians = degrees_to_radians(degrees as f64);
            let back_to_degrees = radians_to_degrees(radians);
            assert!(
                (back_to_degrees - degrees as f64).abs() < 1e-10,
                "Roundtrip conversion failed for {} degrees",
                degrees
            );
        }
    }
}
