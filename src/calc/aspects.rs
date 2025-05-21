/// Aspect types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectType {
    Conjunction = 0,
    Opposition = 1,
    Trine = 2,
    Square = 3,
    Sextile = 4,
    Quincunx = 5,
    SemiSextile = 6,
    SemiSquare = 7,
    Sesquisquare = 8,
    Quintile = 9,
    BiQuintile = 10,
    Septile = 11,
    BiSeptile = 12,
    TriSeptile = 13,
    Novile = 14,
    BiNovile = 15,
    QuadNovile = 16,
}

/// Aspect configuration
#[derive(Debug, Clone)]
pub struct AspectConfig {
    pub orb: f64,
    pub applying: bool,
}

/// Calculate aspects between two positions
pub fn calculate_aspect(
    pos1: f64,
    pos2: f64,
    aspect_type: AspectType,
    orb: f64,
) -> Option<AspectConfig> {
    let aspect_angle = get_aspect_angle(aspect_type);
    let diff = (pos1 - pos2).abs() % 360.0;
    let aspect_diff = (diff - aspect_angle).abs();
    
    if aspect_diff <= orb {
        Some(AspectConfig {
            orb: aspect_diff,
            applying: is_aspect_applying(pos1, pos2, aspect_type),
        })
    } else {
        None
    }
}

/// Get the angle for a given aspect type
fn get_aspect_angle(aspect_type: AspectType) -> f64 {
    match aspect_type {
        AspectType::Conjunction => 0.0,
        AspectType::Opposition => 180.0,
        AspectType::Trine => 120.0,
        AspectType::Square => 90.0,
        AspectType::Sextile => 60.0,
        AspectType::Quincunx => 150.0,
        AspectType::SemiSextile => 30.0,
        AspectType::SemiSquare => 45.0,
        AspectType::Sesquisquare => 135.0,
        AspectType::Quintile => 72.0,
        AspectType::BiQuintile => 144.0,
        AspectType::Septile => 51.428571,
        AspectType::BiSeptile => 102.857143,
        AspectType::TriSeptile => 154.285714,
        AspectType::Novile => 40.0,
        AspectType::BiNovile => 80.0,
        AspectType::QuadNovile => 160.0,
    }
}

/// Check if an aspect is applying (planets moving towards exact aspect)
fn is_aspect_applying(pos1: f64, pos2: f64, aspect_type: AspectType) -> bool {
    let aspect_angle = get_aspect_angle(aspect_type);
    let diff = (pos1 - pos2) % 360.0;
    
    match aspect_type {
        AspectType::Conjunction => diff > 0.0 && diff < 180.0,
        AspectType::Opposition => diff > 0.0 && diff < 180.0,
        AspectType::Trine => diff > 0.0 && diff < 180.0,
        AspectType::Square => diff > 0.0 && diff < 180.0,
        AspectType::Sextile => diff > 0.0 && diff < 180.0,
        AspectType::Quincunx => diff > 0.0 && diff < 180.0,
        AspectType::SemiSextile => diff > 0.0 && diff < 180.0,
        AspectType::SemiSquare => diff > 0.0 && diff < 180.0,
        AspectType::Sesquisquare => diff > 0.0 && diff < 180.0,
        AspectType::Quintile => diff > 0.0 && diff < 180.0,
        AspectType::BiQuintile => diff > 0.0 && diff < 180.0,
        AspectType::Septile => diff > 0.0 && diff < 180.0,
        AspectType::BiSeptile => diff > 0.0 && diff < 180.0,
        AspectType::TriSeptile => diff > 0.0 && diff < 180.0,
        AspectType::Novile => diff > 0.0 && diff < 180.0,
        AspectType::BiNovile => diff > 0.0 && diff < 180.0,
        AspectType::QuadNovile => diff > 0.0 && diff < 180.0,
    }
}

/// Calculate all aspects between a set of positions
pub fn calculate_all_aspects(
    positions: &[f64],
    orbs: &[f64],
    aspect_types: &[AspectType],
) -> Vec<(usize, usize, AspectType, AspectConfig)> {
    let mut aspects = Vec::new();
    
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            for &aspect_type in aspect_types {
                if let Some(config) = calculate_aspect(
                    positions[i],
                    positions[j],
                    aspect_type,
                    orbs[aspect_type as usize],
                ) {
                    aspects.push((i, j, aspect_type, config));
                }
            }
        }
    }
    
    aspects
}

/// Calculate the exact time of an aspect
pub fn calculate_aspect_time(
    pos1: f64,
    vel1: f64,
    pos2: f64,
    vel2: f64,
    aspect_type: AspectType,
) -> Option<f64> {
    let aspect_angle = get_aspect_angle(aspect_type);
    let diff = (pos1 - pos2) % 360.0;
    let vel_diff = vel1 - vel2;
    
    if vel_diff == 0.0 {
        return None;
    }
    
    let time = (aspect_angle - diff) / vel_diff;
    if time >= 0.0 {
        Some(time)
    } else {
        None
    }
} 