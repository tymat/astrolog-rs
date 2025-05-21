use super::super::coordinates::*;
use approx::assert_relative_eq;

#[test]
fn test_ecliptic_to_equatorial() {
    // Test case: Sun at 0° Aries
    let (ra, dec) = ecliptic_to_equatorial(0.0, 0.0, 23.43929111);
    assert_relative_eq!(ra, 0.0, epsilon = 0.0001);
    assert_relative_eq!(dec, 0.0, epsilon = 0.0001);

    // Test case: Sun at 90° (0° Cancer)
    let (ra, dec) = ecliptic_to_equatorial(90.0, 0.0, 23.43929111);
    assert_relative_eq!(ra, 90.0, epsilon = 0.0001);
    assert_relative_eq!(dec, 23.43929111, epsilon = 0.0001);

    // Test case: Sun at 180° (0° Libra)
    let (ra, dec) = ecliptic_to_equatorial(180.0, 0.0, 23.43929111);
    assert_relative_eq!(ra, 180.0, epsilon = 0.0001);
    assert_relative_eq!(dec, 0.0, epsilon = 0.0001);
}

#[test]
fn test_equatorial_to_ecliptic() {
    // Test case: Sun at 0h RA, 0° Dec
    let (lon, lat) = equatorial_to_ecliptic(0.0, 0.0, 23.43929111);
    assert_relative_eq!(lon, 0.0, epsilon = 0.0001);
    assert_relative_eq!(lat, 0.0, epsilon = 0.0001);

    // Test case: Sun at 6h RA, 23.43929111° Dec
    let (lon, lat) = equatorial_to_ecliptic(90.0, 23.43929111, 23.43929111);
    assert_relative_eq!(lon, 90.0, epsilon = 0.0001);
    assert_relative_eq!(lat, 0.0, epsilon = 0.0001);
}

#[test]
fn test_equatorial_to_horizontal() {
    // Test case: Object at zenith
    let (az, alt) = equatorial_to_horizontal(0.0, 0.0, 0.0, 0.0, 0.0);
    assert_relative_eq!(alt, 90.0, epsilon = 0.0001);

    // Test case: Object on celestial equator at local meridian
    let (az, alt) = equatorial_to_horizontal(0.0, 0.0, 0.0, 0.0, 0.0);
    assert_relative_eq!(alt, 0.0, epsilon = 0.0001);
}

#[test]
fn test_calculate_obliquity() {
    // Test case: J2000.0
    let obliquity = calculate_obliquity(2451545.0);
    assert_relative_eq!(obliquity, 23.43929111, epsilon = 0.0001);

    // Test case: Current epoch
    let obliquity = calculate_obliquity(2460000.0);
    assert!(obliquity < 23.43929111); // Should be slightly less than J2000.0
}

#[test]
fn test_calculate_sidereal_time() {
    // Test case: J2000.0 at Greenwich
    let lst = calculate_sidereal_time(2451545.0, 0.0);
    assert_relative_eq!(lst, 280.46061837, epsilon = 0.0001);

    // Test case: J2000.0 at 90° E
    let lst = calculate_sidereal_time(2451545.0, 90.0);
    assert_relative_eq!(lst, 370.46061837, epsilon = 0.0001);
}

#[test]
fn test_calculate_julian_date() {
    // Test case: J2000.0
    let jd = calculate_julian_date(2000, 1, 1, 12, 0, 0.0, 0.0);
    assert_relative_eq!(jd, 2451545.0, epsilon = 0.0001);

    // Test case: 2024-03-20 00:00:00 UTC
    let jd = calculate_julian_date(2024, 3, 20, 0, 0, 0.0, 0.0);
    assert_relative_eq!(jd, 2460390.5, epsilon = 0.0001);

    // Test case: With timezone offset
    let jd = calculate_julian_date(2024, 3, 20, 0, 0, 0.0, -5.0);
    assert_relative_eq!(jd, 2460390.708333, epsilon = 0.0001);
} 