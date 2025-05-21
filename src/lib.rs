pub mod calc;
pub mod core;
pub mod io;

pub use calc::houses::HousePosition;
pub use core::types::HouseSystem;
pub use core::AstrologError;

#[cfg(test)]
mod tests {
    use crate::calc::planets::{Planet, calculate_planet_position};

    #[test]
    fn test_basic_calculations() {
        let jd = 2451545.0; // January 1, 2000
        let position = calculate_planet_position(Planet::Sun, jd).unwrap();
        assert!(position.longitude >= 0.0 && position.longitude < 360.0);
    }
} 