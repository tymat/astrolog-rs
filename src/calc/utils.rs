use chrono::{DateTime, Utc};
use std::f64::consts::PI;

/// Converts a date to Julian date.
///
/// The Julian date is a continuous count of days since noon Universal Time
/// on January 1, 4713 BCE (proleptic Julian calendar).
///
/// # Arguments
///
/// * `datetime` - The date and time as a DateTime<Utc>
///
/// # Returns
///
/// The Julian date as a floating-point number
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::utils::date_to_julian;
/// use chrono::{DateTime, Utc};
///
/// let datetime = Utc::now();
/// let jd = date_to_julian(datetime);
/// println!("Julian date: {}", jd);
/// ```
#[allow(dead_code)]
pub fn date_to_julian(datetime: chrono::DateTime<chrono::Utc>) -> f64 {
    let unix_timestamp = datetime.timestamp() as f64;
    (unix_timestamp / 86400.0) + 2440587.5
}

/// Calculate Julian centuries since J2000.0
#[allow(dead_code)]
pub fn julian_centuries(julian_date: f64) -> f64 {
    (julian_date - 2451545.0) / 36525.0
}

/// Normalizes an angle to the range [0, 360).
///
/// This function takes an angle in degrees and ensures it falls within
/// the range of 0 to 360 degrees by adding or subtracting multiples of 360.
///
/// # Arguments
///
/// * `angle` - The angle in degrees
///
/// # Returns
///
/// The normalized angle in degrees (0 ≤ angle < 360)
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::utils::normalize_angle;
///
/// assert_eq!(normalize_angle(370.0), 10.0);
/// assert_eq!(normalize_angle(-10.0), 350.0);
/// assert_eq!(normalize_angle(360.0), 0.0);
/// ```
#[allow(dead_code)]
pub fn normalize_angle(angle: f64) -> f64 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

/// Converts degrees to radians.
///
/// This function converts an angle from degrees to radians.
/// The conversion is done by multiplying the degrees by π/180.
///
/// # Arguments
///
/// * `degrees` - The angle in degrees
///
/// # Returns
///
/// The angle in radians
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::utils::degrees_to_radians;
///
/// let radians = degrees_to_radians(180.0);
/// assert!((radians - std::f64::consts::PI).abs() < 1e-10);
/// ```
#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Converts radians to degrees.
///
/// This function converts an angle from radians to degrees.
/// The conversion is done by multiplying the radians by 180/π.
///
/// # Arguments
///
/// * `radians` - The angle in radians
///
/// # Returns
///
/// The angle in degrees
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::utils::radians_to_degrees;
///
/// let degrees = radians_to_degrees(std::f64::consts::PI);
/// assert!((degrees - 180.0).abs() < 1e-10);
/// ```
#[allow(dead_code)]
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
