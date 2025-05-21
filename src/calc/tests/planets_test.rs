use super::super::planets::*;
use crate::core::AstrologError;

#[test]
fn test_planet_enum() {
    // Test that planet values match their indices
    assert_eq!(Planet::Sun as usize, 0);
    assert_eq!(Planet::Moon as usize, 1);
    assert_eq!(Planet::Mercury as usize, 2);
    assert_eq!(Planet::Venus as usize, 3);
    assert_eq!(Planet::Mars as usize, 4);
    assert_eq!(Planet::Jupiter as usize, 5);
    assert_eq!(Planet::Saturn as usize, 6);
    assert_eq!(Planet::Uranus as usize, 7);
    assert_eq!(Planet::Neptune as usize, 8);
    assert_eq!(Planet::Pluto as usize, 9);
    assert_eq!(Planet::MeanNode as usize, 10);
    assert_eq!(Planet::TrueNode as usize, 11);
    assert_eq!(Planet::MeanLilith as usize, 12);
    assert_eq!(Planet::TrueLilith as usize, 13);
    assert_eq!(Planet::Chiron as usize, 14);
}

#[test]
fn test_calculate_planet_positions() {
    // Test J2000.0 epoch
    let positions = calculate_planet_positions(2451545.0).unwrap();
    assert_eq!(positions.len(), 15);
    
    // Test that all positions have valid values
    for pos in positions {
        assert!(pos.longitude >= 0.0 && pos.longitude < 360.0);
        assert!(pos.latitude >= -90.0 && pos.latitude <= 90.0);
        assert!(pos.distance > 0.0);
    }
}

#[test]
fn test_calculate_retrogrades() {
    // Test case: All planets direct
    let positions = vec![
        PlanetPosition {
            longitude: 0.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 90.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let retrogrades = calculate_retrogrades(&positions);
    assert_eq!(retrogrades, vec![false, false]);
    
    // Test case: One planet retrograde
    let positions = vec![
        PlanetPosition {
            longitude: 0.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 90.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: -0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let retrogrades = calculate_retrogrades(&positions);
    assert_eq!(retrogrades, vec![false, true]);
}

#[test]
fn test_calculate_stations() {
    // Test case: No stations
    let positions = vec![
        PlanetPosition {
            longitude: 0.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 90.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let prev_positions = vec![
        PlanetPosition {
            longitude: 359.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 89.5,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let stations = calculate_stations(&positions, &prev_positions);
    assert_eq!(stations, vec![false, false]);
    
    // Test case: One station
    let positions = vec![
        PlanetPosition {
            longitude: 0.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: -1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 90.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let prev_positions = vec![
        PlanetPosition {
            longitude: 1.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 89.5,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let stations = calculate_stations(&positions, &prev_positions);
    assert_eq!(stations, vec![true, false]);
}

#[test]
fn test_calculate_planetary_aspects() {
    let positions = vec![
        PlanetPosition {
            longitude: 0.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 1.0,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 60.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
        PlanetPosition {
            longitude: 90.0,
            latitude: 0.0,
            distance: 1.0,
            speed_longitude: 0.5,
            speed_latitude: 0.0,
            speed_distance: 0.0,
        },
    ];
    
    let orbs = vec![10.0, 10.0, 10.0, 10.0, 10.0];
    
    let aspects = calculate_planetary_aspects(&positions, &orbs);
    assert!(!aspects.is_empty());
} 