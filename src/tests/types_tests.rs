use crate::core::{ChartInfo, HouseSystem};
use chrono::{DateTime, Utc, TimeZone};
use std::str::FromStr;

#[test]
fn test_house_system_from_str() {
    let test_cases = [
        ("placidus", HouseSystem::Placidus),
        ("koch", HouseSystem::Koch),
        ("equal", HouseSystem::Equal),
        ("wholesign", HouseSystem::WholeSign),
        ("campanus", HouseSystem::Campanus),
        ("regiomontanus", HouseSystem::Regiomontanus),
        ("meridian", HouseSystem::Meridian),
        ("alcabitius", HouseSystem::Alcabitius),
        ("morinus", HouseSystem::Morinus),
        ("krusinski", HouseSystem::Krusinski),
    ];

    for (input, expected) in test_cases.iter() {
        let result = HouseSystem::from_str(input).unwrap();
        assert_eq!(result, *expected, "Failed to parse house system: {}", input);
    }
}

#[test]
fn test_invalid_house_system() {
    let invalid_systems = ["invalid", "placidusss", "kochh", "equall"];
    
    for system in invalid_systems.iter() {
        let result = HouseSystem::from_str(system);
        assert!(result.is_err(), "Should fail to parse invalid house system: {}", system);
    }
}

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

#[test]
fn test_chart_info_validation() {
    // Test valid coordinates
    let valid_info = ChartInfo {
        date: Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap(),
        timezone: 0.0,
        latitude: 90.0,  // North Pole
        longitude: 0.0,
        house_system: HouseSystem::Placidus,
    };
    assert_eq!(valid_info.latitude, 90.0);

    // Test valid timezone
    let valid_tz_info = ChartInfo {
        date: Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap(),
        timezone: 12.0,  // UTC+12
        latitude: 0.0,
        longitude: 0.0,
        house_system: HouseSystem::Placidus,
    };
    assert_eq!(valid_tz_info.timezone, 12.0);
} 