use chrono::{DateTime, Utc, TimeZone};
use crate::core::{ChartInfo, HouseSystem};
use crate::calc::planets::{Planet, PlanetPosition};
use crate::calc::houses::HousePosition;

#[cfg(test)]
mod tests {
    use super::*;

    // Test data from the example astrolog output
    const TEST_DATE: &str = "1977-10-24T04:56:00Z";
    const TEST_LATITUDE: f64 = 14.65;  // 14:38:55N
    const TEST_LONGITUDE: f64 = 121.05; // 121:03:03E
    const TEST_TIMEZONE: f64 = 0.0;    // ST Zone 0W

    // Expected planet positions from the example output
    const EXPECTED_PLANET_POSITIONS: &[(Planet, f64, f64, bool)] = &[
        (Planet::Sun, 210.674, 0.995, false),    // Sun : 210.674 +0.995
        (Planet::Moon, 358.595, 12.82, false),   // Moon: 358.595 +12.82
        (Planet::Mercury, 214.148, 1.632, false), // Merc: 214.148 +1.632
        (Planet::Venus, 188.853, 1.242, true),   // Venu: 188.853 +1.242 (R)
        (Planet::Mars, 118.878, 0.440, false),   // Mars: 118.878 +0.440
        (Planet::Jupiter, 96.142, 0.000, false), // Jupi: 96.142 +0.000
        (Planet::Saturn, 148.485, 0.080, false), // Satu: 148.485 +0.080
        (Planet::Uranus, 221.400, 0.061, false), // Uran: 221.400 +0.061
        (Planet::Neptune, 254.296, 0.029, false), // Nept: 254.296 +0.029
        (Planet::Pluto, 194.736, 0.038, false),  // Plut: 194.736 +0.038
    ];

    // Expected house cusps from the example output
    const EXPECTED_HOUSE_CUSPS: &[f64] = &[
        310.315, // House cusp  1: 310.315
        340.315, // House cusp  2: 340.315
        10.315,  // House cusp  3:  10.315
        40.315,  // House cusp  4:  40.315
        70.315,  // House cusp  5:  70.315
        100.315, // House cusp  6: 100.315
        130.315, // House cusp  7: 130.315
        160.315, // House cusp  8: 160.315
        190.315, // House cusp  9: 190.315
        220.315, // House cusp 10: 220.315
        250.315, // House cusp 11: 250.315
        280.315, // House cusp 12: 280.315
    ];

    #[test]
    fn test_chart_generation() {
        // Create chart info from test data
        let date = Utc.datetime_from_str(TEST_DATE, "%Y-%m-%dT%H:%M:%SZ")
            .expect("Failed to parse test date");
        
        let chart_info = ChartInfo {
            date,
            timezone: TEST_TIMEZONE,
            latitude: TEST_LATITUDE,
            longitude: TEST_LONGITUDE,
            house_system: HouseSystem::Placidus,
        };

        // TODO: Generate chart using our implementation
        // let chart = generate_chart(&chart_info);

        // Verify planet positions
        // for (planet, expected_longitude, expected_speed, expected_retrograde) in EXPECTED_PLANET_POSITIONS {
        //     let position = chart.get_planet_position(*planet);
        //     assert_relative_eq!(position.longitude, *expected_longitude, epsilon = 1e-3);
        //     assert_relative_eq!(position.speed, *expected_speed, epsilon = 1e-3);
        //     assert_eq!(position.is_retrograde, *expected_retrograde);
        // }

        // Verify house cusps
        // for (i, expected_cusp) in EXPECTED_HOUSE_CUSPS.iter().enumerate() {
        //     let cusp = chart.get_house_cusp(i + 1);
        //     assert_relative_eq!(cusp, *expected_cusp, epsilon = 1e-3);
        // }
    }

    #[test]
    fn test_chart_info_creation() {
        let date = Utc.datetime_from_str(TEST_DATE, "%Y-%m-%dT%H:%M:%SZ")
            .expect("Failed to parse test date");
        
        let chart_info = ChartInfo {
            date,
            timezone: TEST_TIMEZONE,
            latitude: TEST_LATITUDE,
            longitude: TEST_LONGITUDE,
            house_system: HouseSystem::Placidus,
        };

        assert_eq!(chart_info.latitude, TEST_LATITUDE);
        assert_eq!(chart_info.longitude, TEST_LONGITUDE);
        assert_eq!(chart_info.timezone, TEST_TIMEZONE);
        assert_eq!(chart_info.house_system, HouseSystem::Placidus);
    }
} 