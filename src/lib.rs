pub mod api;
pub mod calc;
pub mod charts;
pub mod core;
pub mod io;
pub mod utils;

#[cfg(test)]
pub mod tests {
    pub mod api_tests;
    pub mod functional;
    pub mod chart_tests;
    pub mod types_tests;
    pub mod utils_tests;

    use super::*;
    use crate::calc::swiss_ephemeris;
    use approx::assert_relative_eq;

    // Natal chart data: October 24, 1977, 04:56 AM, 121:03:03E 14:38:55N
    const TEST_YEAR: i32 = 1977;
    const TEST_MONTH: i32 = 10;
    const TEST_DAY: i32 = 24;
    const TEST_HOUR: f64 = 4.0 + 56.0 / 60.0; // 04:56 AM

    fn setup() -> Result<(), String> {
        swiss_ephemeris::init_swiss_ephemeris()
            .map_err(|e| format!("Failed to initialize Swiss Ephemeris: {}", e))
    }

    #[test]
    fn test_basic_calculations() -> Result<(), String> {
        setup()?;
        let sun_pos =
            calculate_planet_position(Planet::Sun, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Sun position: {}", e))?;
        let moon_pos =
            calculate_planet_position(Planet::Moon, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
                .map_err(|e| format!("Failed to calculate Moon position: {}", e))?;
        // Update expected value for Sun's longitude
        assert_relative_eq!(sun_pos.longitude, 210.674, epsilon = 1e-3);
        assert!(sun_pos.longitude >= 0.0 && sun_pos.longitude < 360.0);
        assert!(moon_pos.longitude >= 0.0 && moon_pos.longitude < 360.0);
        Ok(())
    }
}

pub use calc::houses::HousePosition;
pub use calc::planets::{calculate_planet_position, Planet, PlanetPosition};
pub use core::types::HouseSystem;
pub use core::AstrologError;
