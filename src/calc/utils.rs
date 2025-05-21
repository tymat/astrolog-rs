use std::f64::consts::PI;

pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f64) -> f64 {
    radians * 180.0 / PI
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
            assert!((result - expected).abs() < 1e-10, 
                "normalize_angle({}) = {}, expected {}", input, result, expected);
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
            assert!((result - expected).abs() < 1e-10,
                "degrees_to_radians({}) = {}, expected {}", degrees, result, expected);
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
            assert!((result - expected).abs() < 1e-10,
                "radians_to_degrees({}) = {}, expected {}", radians, result, expected);
        }
    }

    #[test]
    fn test_angle_conversion_roundtrip() {
        for degrees in 0..360 {
            let radians = degrees_to_radians(degrees as f64);
            let back_to_degrees = radians_to_degrees(radians);
            assert!((back_to_degrees - degrees as f64).abs() < 1e-10,
                "Roundtrip conversion failed for {} degrees", degrees);
        }
    }
} 