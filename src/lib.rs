pub mod calc;
pub mod core;
pub mod io;
pub mod api;

pub use calc::houses::HousePosition;
pub use core::types::HouseSystem;
pub use core::AstrologError;
pub use calc::planets::{Planet, PlanetPosition, calculate_planet_position};

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::calc::swiss_ephemeris;

    // Natal chart data: October 24, 1977, 04:56 AM, 121:03:03E 14:38:55N
    const TEST_YEAR: i32 = 1977;
    const TEST_MONTH: i32 = 10;
    const TEST_DAY: i32 = 24;
    const TEST_HOUR: f64 = 4.0 + 56.0/60.0; // 04:56 AM

    fn setup() -> Result<(), String> {
        swiss_ephemeris::init_swiss_ephemeris()
            .map_err(|e| format!("Failed to initialize Swiss Ephemeris: {}", e))
    }

    #[test]
    fn test_basic_calculations() -> Result<(), String> {
        setup()?;
        let sun_pos = calculate_planet_position(Planet::Sun, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
            .map_err(|e| format!("Failed to calculate Sun position: {}", e))?;
        let moon_pos = calculate_planet_position(Planet::Moon, TEST_YEAR, TEST_MONTH, TEST_DAY, TEST_HOUR)
            .map_err(|e| format!("Failed to calculate Moon position: {}", e))?;
        // Update expected value for Sun's longitude
        assert_relative_eq!(sun_pos.longitude, 210.674, epsilon = 1e-3);
        assert!(sun_pos.longitude >= 0.0 && sun_pos.longitude < 360.0);
        assert!(moon_pos.longitude >= 0.0 && moon_pos.longitude < 360.0);
        Ok(())
    }
} 