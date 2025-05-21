pub mod api;
pub mod calc;
pub mod core;
pub mod io;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ChartInfo, HouseSystem};
    use chrono::{DateTime, Utc, TimeZone};

    #[test]
    fn test_basic_chart_generation() {
        let info = ChartInfo {
            date: Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap(),
            timezone: 0.0,
            latitude: 51.5074, // London
            longitude: -0.1278,
            house_system: HouseSystem::Placidus,
        };

        assert_eq!(info.latitude, 51.5074);
        assert_eq!(info.longitude, -0.1278);
        assert_eq!(info.house_system, HouseSystem::Placidus);
    }
} 