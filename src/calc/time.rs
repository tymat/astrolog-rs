pub fn julian_centuries(julian_date: f64) -> f64 {
    (julian_date - 2451545.0) / 36525.0
} 