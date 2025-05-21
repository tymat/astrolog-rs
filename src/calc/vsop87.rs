use std::f64::consts::PI;

/// Calculate the Julian centuries from J2000.0
pub fn julian_centuries(julian_date: f64) -> f64 {
    (julian_date - 2451545.0) / 36525.0
}

/// Calculate the mean anomaly for a planet
pub fn mean_anomaly(t: f64, a: f64, b: f64, c: f64) -> f64 {
    let mut m = a + b * t + c * t * t;
    m = m % (2.0 * PI);
    if m < 0.0 {
        m += 2.0 * PI;
    }
    m
}

/// Calculate the eccentricity of a planet's orbit
pub fn eccentricity(t: f64, a: f64, b: f64, c: f64) -> f64 {
    a + b * t + c * t * t
}

/// Calculate the semi-major axis of a planet's orbit
pub fn semi_major_axis(t: f64, a: f64, b: f64, c: f64) -> f64 {
    a + b * t + c * t * t
}

/// Calculate the inclination of a planet's orbit
pub fn inclination(t: f64, a: f64, b: f64, c: f64) -> f64 {
    a + b * t + c * t * t
}

/// Calculate the longitude of the ascending node
pub fn ascending_node(t: f64, a: f64, b: f64, c: f64) -> f64 {
    let mut node = a + b * t + c * t * t;
    node = node % (2.0 * PI);
    if node < 0.0 {
        node += 2.0 * PI;
    }
    node
}

/// Calculate the argument of perihelion
pub fn perihelion(t: f64, a: f64, b: f64, c: f64) -> f64 {
    let mut peri = a + b * t + c * t * t;
    peri = peri % (2.0 * PI);
    if peri < 0.0 {
        peri += 2.0 * PI;
    }
    peri
}

/// Calculate the true anomaly from mean anomaly and eccentricity
pub fn true_anomaly(mean_anomaly: f64, eccentricity: f64) -> f64 {
    let mut e = mean_anomaly;
    let mut delta = 1.0;
    let mut iterations = 0;
    
    while delta.abs() > 1e-12 && iterations < 50 {
        let e_next = e - (e - eccentricity * e.sin() - mean_anomaly) / (1.0 - eccentricity * e.cos());
        delta = e_next - e;
        e = e_next;
        iterations += 1;
    }
    
    let true_anomaly = 2.0 * ((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt() * (e / 2.0).tan();
    true_anomaly.atan2(1.0)
}

/// Calculate the heliocentric coordinates of a planet
pub fn heliocentric_coordinates(
    t: f64,
    a: f64,
    e: f64,
    i: f64,
    l: f64,
    lp: f64,
    node: f64,
) -> (f64, f64, f64) {
    // Convert angles to radians
    let i_rad = i * PI / 180.0;
    let node_rad = node * PI / 180.0;
    let lp_rad = lp * PI / 180.0;
    
    // Calculate mean anomaly
    let m = mean_anomaly(t, l, 0.0, 0.0);
    
    // Calculate true anomaly
    let v = true_anomaly(m, e);
    
    // Calculate radius vector
    let r = a * (1.0 - e * e) / (1.0 + e * v.cos());
    
    // Calculate heliocentric coordinates in the orbital plane
    let x_orb = r * v.cos();
    let y_orb = r * v.sin();
    
    // Transform to ecliptic coordinates
    let x = x_orb * (node_rad).cos() - y_orb * (node_rad).sin() * i_rad.cos();
    let y = x_orb * (node_rad).sin() + y_orb * (node_rad).cos() * i_rad.cos();
    let z = y_orb * i_rad.sin();
    
    (x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_julian_centuries() {
        let jd = 2451545.0; // J2000.0
        assert_relative_eq!(julian_centuries(jd), 0.0);
        
        let jd = 2451545.0 + 36525.0; // One century later
        assert_relative_eq!(julian_centuries(jd), 1.0);
    }

    #[test]
    fn test_mean_anomaly() {
        let t = 0.0; // J2000.0
        let m = mean_anomaly(t, 0.0, 1.0, 0.0);
        assert_relative_eq!(m, 0.0);
        
        let m = mean_anomaly(t, PI, 1.0, 0.0);
        assert_relative_eq!(m, PI);
    }

    #[test]
    fn test_true_anomaly() {
        let m = 0.0;
        let e = 0.0;
        let v = true_anomaly(m, e);
        assert_relative_eq!(v, 0.0);
        
        let m = PI;
        let e = 0.0;
        let v = true_anomaly(m, e);
        assert_relative_eq!(v, PI);
    }

    #[test]
    fn test_heliocentric_coordinates_with_inclination() {
        let t = 0.0; // J2000.0
        let (x, y, z) = heliocentric_coordinates(
            t,
            1.0,    // Semi-major axis
            0.0,    // Eccentricity
            45.0,   // Inclination (45 degrees)
            0.0,    // Mean longitude
            0.0,    // Longitude of perihelion
            0.0,    // Longitude of ascending node
        );
        
        // For 45-degree inclination, z should be significant
        assert!(z.abs() > 0.0);
        
        // Test with zero inclination
        let (x, y, z) = heliocentric_coordinates(
            t,
            1.0,    // Semi-major axis
            0.0,    // Eccentricity
            0.0,    // Inclination
            0.0,    // Mean longitude
            0.0,    // Longitude of perihelion
            0.0,    // Longitude of ascending node
        );
        
        // For zero inclination, z should be zero
        assert_relative_eq!(z, 0.0, epsilon = 1e-10);
    }
} 