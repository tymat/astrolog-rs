pub mod types;
pub mod calc;

use chrono::{DateTime, Utc};
pub use types::*;
pub use calc::HouseSystem;

/// Information needed to generate an astrological chart
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChartInfo {
    /// The date and time for the chart
    pub date: DateTime<Utc>,
    /// Latitude in degrees (positive for North, negative for South)
    pub latitude: f64,
    /// Longitude in degrees (positive for East, negative for West)
    pub longitude: f64,
    /// Timezone offset in hours from UTC
    pub timezone: f64,
    /// The house system to use for the chart
    pub house_system: HouseSystem,
}

/// Positions of celestial bodies and house cusps in a chart
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChartPositions {
    /// Longitudes of celestial bodies in degrees
    pub zodiac_positions: Vec<f64>,
    /// House numbers (1-12) for each celestial body
    pub house_placements: Vec<u8>,
    /// Longitudes of house cusps in degrees
    pub house_cusps: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_chart_info_creation() {
        let info = ChartInfo {
            date: Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap(),
            timezone: 0.0,
            latitude: 51.5074,
            longitude: -0.1278,
            house_system: HouseSystem::Placidus,
        };

        assert_eq!(info.latitude, 51.5074);
        assert_eq!(info.longitude, -0.1278);
        assert_eq!(info.timezone, 0.0);
        assert_eq!(info.house_system, HouseSystem::Placidus);
    }
} 