use crate::calc::utils;

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
        let result = utils::normalize_angle(*input);
        assert!((result - expected).abs() < 1e-10, 
            "normalize_angle({}) = {}, expected {}", input, result, expected);
    }
}

#[test]
fn test_degrees_to_radians() {
    let test_cases = [
        (0.0, 0.0),
        (90.0, std::f64::consts::PI / 2.0),
        (180.0, std::f64::consts::PI),
        (270.0, 3.0 * std::f64::consts::PI / 2.0),
        (360.0, 2.0 * std::f64::consts::PI),
    ];

    for (degrees, expected) in test_cases.iter() {
        let result = utils::degrees_to_radians(*degrees);
        assert!((result - expected).abs() < 1e-10,
            "degrees_to_radians({}) = {}, expected {}", degrees, result, expected);
    }
}

#[test]
fn test_radians_to_degrees() {
    let test_cases = [
        (0.0, 0.0),
        (std::f64::consts::PI / 2.0, 90.0),
        (std::f64::consts::PI, 180.0),
        (3.0 * std::f64::consts::PI / 2.0, 270.0),
        (2.0 * std::f64::consts::PI, 360.0),
    ];

    for (radians, expected) in test_cases.iter() {
        let result = utils::radians_to_degrees(*radians);
        assert!((result - expected).abs() < 1e-10,
            "radians_to_degrees({}) = {}, expected {}", radians, result, expected);
    }
}

#[test]
fn test_angle_conversion_roundtrip() {
    for degrees in 0..360 {
        let radians = utils::degrees_to_radians(degrees as f64);
        let back_to_degrees = utils::radians_to_degrees(radians);
        assert!((back_to_degrees - degrees as f64).abs() < 1e-10,
            "Roundtrip conversion failed for {} degrees", degrees);
    }
} 