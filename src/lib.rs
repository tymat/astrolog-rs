pub mod calc;
pub mod core;
pub mod io;

#[cfg(test)]
mod tests {
    use crate::calc::planets::{Planet, calculate_planet_position};
    use crate::core::types::HouseSystem;
    use crate::calc::aspects::calculate_aspect;

    #[test]
    fn test_basic_calculations() {
        let jd = 2451545.0; // January 1, 2000
        let position = calculate_planet_position(Planet::Sun, jd).unwrap();
        assert!(position.longitude >= 0.0 && position.longitude < 360.0);
    }
} 