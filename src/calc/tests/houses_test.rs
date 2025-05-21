use super::super::houses::*;
use crate::core::{ChartInfo, ChartPositions};

#[test]
fn test_normalize_angle() {
    assert_eq!(normalize_angle(0.0), 0.0);
    assert_eq!(normalize_angle(360.0), 0.0);
    assert_eq!(normalize_angle(720.0), 0.0);
    assert_eq!(normalize_angle(-360.0), 0.0);
    assert_eq!(normalize_angle(180.0), 180.0);
    assert_eq!(normalize_angle(540.0), 180.0);
    assert_eq!(normalize_angle(-180.0), 180.0);
}

#[test]
fn test_find_house() {
    // Test case: Position in middle of first house
    let house_cusps = vec![0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0];
    assert_eq!(find_house(15.0, &house_cusps), 1);
    
    // Test case: Position at house cusp
    assert_eq!(find_house(30.0, &house_cusps), 2);
    
    // Test case: Position in last house
    assert_eq!(find_house(345.0, &house_cusps), 12);
    
    // Test case: Position at 0° Aries
    assert_eq!(find_house(0.0, &house_cusps), 1);
    
    // Test case: Position at 359°59'
    assert_eq!(find_house(359.99, &house_cusps), 12);
}

#[test]
fn test_find_house_with_irregular_cusps() {
    // Test case: House cusps not in order
    let house_cusps = vec![
        350.0, // 12th house
        20.0,  // 1st house
        50.0,  // 2nd house
        80.0,  // 3rd house
        110.0, // 4th house
        140.0, // 5th house
        170.0, // 6th house
        200.0, // 7th house
        230.0, // 8th house
        260.0, // 9th house
        290.0, // 10th house
        320.0, // 11th house
    ];
    
    // Test position in 12th house
    assert_eq!(find_house(355.0, &house_cusps), 12);
    
    // Test position in 1st house
    assert_eq!(find_house(10.0, &house_cusps), 1);
    
    // Test position at house cusp
    assert_eq!(find_house(20.0, &house_cusps), 2);
}

#[test]
fn test_calculate_house_placements() {
    let mut positions = ChartPositions {
        zodiac_positions: vec![0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0],
        house_placements: vec![0; 12],
        house_cusps: vec![0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0],
    };
    
    calculate_house_placements(&mut positions);
    
    // Check that each position is placed in the correct house
    for i in 0..12 {
        assert_eq!(positions.house_placements[i], (i + 1) as u8);
    }
}

#[test]
fn test_house_system_enum() {
    // Test that house system values match their indices
    assert_eq!(HouseSystem::Placidus as usize, 0);
    assert_eq!(HouseSystem::Koch as usize, 1);
    assert_eq!(HouseSystem::Equal as usize, 2);
    assert_eq!(HouseSystem::EqualMidheaven as usize, 3);
    assert_eq!(HouseSystem::Whole as usize, 4);
    assert_eq!(HouseSystem::Meridian as usize, 5);
    assert_eq!(HouseSystem::Alcabitius as usize, 6);
    assert_eq!(HouseSystem::Porphyry as usize, 7);
    assert_eq!(HouseSystem::Regiomontanus as usize, 8);
    assert_eq!(HouseSystem::Campanus as usize, 9);
    assert_eq!(HouseSystem::Morinus as usize, 10);
    assert_eq!(HouseSystem::Krusinski as usize, 11);
    assert_eq!(HouseSystem::Vedic as usize, 12);
} 