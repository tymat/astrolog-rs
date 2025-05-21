use crate::core::{AstrologError, ChartInfo, ChartPositions};
use serde::{Serialize, Deserialize};

/// House system types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HouseSystem {
    Placidus = 0,
    Koch = 1,
    Equal = 2,
    EqualMidheaven = 3,
    Whole = 4,
    Meridian = 5,
    Alcabitius = 6,
    Porphyry = 7,
    Regiomontanus = 8,
    Campanus = 9,
    Morinus = 10,
    Krusinski = 11,
    Vedic = 12,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HousePosition {
    pub cusps: [f64; 12],  // House cusps in degrees (1-12)
    pub system: HouseSystem, // House system used
}

impl HousePosition {
    pub fn new(cusps: [f64; 12], system: HouseSystem) -> Self {
        Self { cusps, system }
    }

    pub fn get_cusp(&self, house: u8) -> Option<f64> {
        if house >= 1 && house <= 12 {
            Some(self.cusps[(house - 1) as usize])
        } else {
            None
        }
    }
}

/// Calculate house cusps for a given date, time, and location
pub fn calculate_houses(
    julian_date: f64,
    latitude: f64,
    longitude: f64,
    system: HouseSystem,
) -> Result<HousePosition, AstrologError> {
    // TODO: Implement house calculation based on the selected system
    // For now, return a placeholder implementation
    Ok(HousePosition::new([0.0; 12], system))
}

/// Calculate house cusps for a given chart
pub fn calculate_houses_for_chart(
    chart_info: &ChartInfo,
    positions: &mut ChartPositions,
    system: HouseSystem,
) -> Result<(), AstrologError> {
    match system {
        HouseSystem::Placidus => calculate_placidus(chart_info, positions),
        HouseSystem::Koch => calculate_koch(chart_info, positions),
        HouseSystem::Equal => calculate_equal(chart_info, positions),
        HouseSystem::EqualMidheaven => calculate_equal_midheaven(chart_info, positions),
        HouseSystem::Whole => calculate_whole(chart_info, positions),
        HouseSystem::Meridian => calculate_meridian(chart_info, positions),
        HouseSystem::Alcabitius => calculate_alcabitius(chart_info, positions),
        HouseSystem::Porphyry => calculate_porphyry(chart_info, positions),
        HouseSystem::Regiomontanus => calculate_regiomontanus(chart_info, positions),
        HouseSystem::Campanus => calculate_campanus(chart_info, positions),
        HouseSystem::Morinus => calculate_morinus(chart_info, positions),
        HouseSystem::Krusinski => calculate_krusinski(chart_info, positions),
        HouseSystem::Vedic => calculate_vedic(chart_info, positions),
    }
}

/// Calculate house placements for all objects
pub fn calculate_house_placements(positions: &mut ChartPositions) {
    for i in 0..positions.zodiac_positions.len() {
        positions.house_placements[i] = find_house(positions.zodiac_positions[i], &positions.house_cusps);
    }
}

/// Find which house a position falls into
fn find_house(position: f64, house_cusps: &[f64]) -> u8 {
    let position = normalize_angle(position);
    
    for i in 0..12 {
        let next = (i + 1) % 12;
        if house_cusps[i] <= house_cusps[next] {
            if position >= house_cusps[i] && position < house_cusps[next] {
                return i as u8 + 1;
            }
        } else {
            // Handle case where house spans 0° Aries
            if position >= house_cusps[i] || position < house_cusps[next] {
                return i as u8 + 1;
            }
        }
    }
    
    1 // Default to first house if something goes wrong
}

/// Normalize an angle to 0-360 degrees
fn normalize_angle(angle: f64) -> f64 {
    let mut angle = angle % 360.0;
    if angle < 0.0 {
        angle += 360.0;
    }
    angle
}

// House system calculations
fn calculate_placidus(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Placidus house system
    Err(AstrologError::NotImplemented("Placidus house system not yet implemented".into()))
}

fn calculate_koch(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Koch house system
    Err(AstrologError::NotImplemented("Koch house system not yet implemented".into()))
}

fn calculate_equal(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Equal house system
    Err(AstrologError::NotImplemented("Equal house system not yet implemented".into()))
}

fn calculate_equal_midheaven(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Equal Midheaven house system
    Err(AstrologError::NotImplemented("Equal Midheaven house system not yet implemented".into()))
}

fn calculate_whole(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Whole house system
    Err(AstrologError::NotImplemented("Whole house system not yet implemented".into()))
}

fn calculate_meridian(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Meridian house system
    Err(AstrologError::NotImplemented("Meridian house system not yet implemented".into()))
}

fn calculate_alcabitius(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Alcabitius house system
    Err(AstrologError::NotImplemented("Alcabitius house system not yet implemented".into()))
}

fn calculate_porphyry(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Porphyry house system
    Err(AstrologError::NotImplemented("Porphyry house system not yet implemented".into()))
}

fn calculate_regiomontanus(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Regiomontanus house system
    Err(AstrologError::NotImplemented("Regiomontanus house system not yet implemented".into()))
}

fn calculate_campanus(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Campanus house system
    Err(AstrologError::NotImplemented("Campanus house system not yet implemented".into()))
}

fn calculate_morinus(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Morinus house system
    Err(AstrologError::NotImplemented("Morinus house system not yet implemented".into()))
}

fn calculate_krusinski(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Krusinski house system
    Err(AstrologError::NotImplemented("Krusinski house system not yet implemented".into()))
}

fn calculate_vedic(chart_info: &ChartInfo, positions: &mut ChartPositions) -> Result<(), AstrologError> {
    // TODO: Implement Vedic house system
    Err(AstrologError::NotImplemented("Vedic house system not yet implemented".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_house_position_creation() {
        let cusps = [0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0];
        let position = HousePosition::new(cusps, HouseSystem::Placidus);
        
        for i in 0..12 {
            assert_relative_eq!(position.cusps[i], (i as f64) * 30.0);
        }
        assert_eq!(position.system, HouseSystem::Placidus);
    }

    #[test]
    fn test_get_cusp() {
        let cusps = [0.0, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0];
        let position = HousePosition::new(cusps, HouseSystem::Placidus);
        
        assert_eq!(position.get_cusp(1), Some(0.0));
        assert_eq!(position.get_cusp(12), Some(330.0));
        assert_eq!(position.get_cusp(0), None);
        assert_eq!(position.get_cusp(13), None);
    }
} 