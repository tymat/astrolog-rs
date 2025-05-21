// use crate::calc::utils::normalize_angle;
use crate::calc::PlanetPosition;

/// Aspect types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AspectType {
    Conjunction,    // 0°
    SemiSextile,    // 30°
    SemiSquare,     // 45°
    Sextile,        // 60°
    Quintile,       // 72°
    Square,         // 90°
    BiQuintile,     // 144°
    Trine,          // 120°
    Sesquisquare,   // 135°
    Quincunx,       // 150°
    Opposition,     // 180°
    Septile,        // 51.428571°
    BiSeptile,      // 102.857143°
    TriSeptile,     // 154.285714°
    Novile,         // 40°
    BiNovile,       // 80°
    QuadNovile,     // 160°
}

/// Aspect configuration
#[allow(dead_code)]
pub struct AspectConfig {
    pub orb: f64,
    pub applying: bool,
}

/// Calculate aspects between two positions
#[allow(dead_code)]
pub fn calculate_aspect(
    pos1: f64,
    pos2: f64,
    aspect_type: AspectType,
    orb: f64,
) -> Option<AspectConfig> {
    let _aspect_angle = get_aspect_angle(aspect_type);
    let diff = (pos1 - pos2).abs() % 360.0;
    let aspect_diff = (diff - _aspect_angle).abs();
    
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
#[allow(dead_code)]
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
#[allow(dead_code)]
fn is_aspect_applying(pos1: f64, pos2: f64, aspect_type: AspectType) -> bool {
    let _aspect_angle = get_aspect_angle(aspect_type);
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn calculate_aspect_time(
    pos1: f64,
    vel1: f64,
    pos2: f64,
    vel2: f64,
    aspect_type: AspectType,
) -> Option<f64> {
    let _aspect_angle = get_aspect_angle(aspect_type);
    let diff = (pos1 - pos2) % 360.0;
    let vel_diff = vel1 - vel2;
    
    if vel_diff == 0.0 {
        return None;
    }
    
    let time = (diff - _aspect_angle) / vel_diff;
    if time >= 0.0 {
        Some(time)
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Aspect {
    pub planet1: String,
    pub planet2: String,
    pub aspect_type: AspectType,
    pub orb: f64,
}

impl AspectType {
    pub fn angle(&self) -> f64 {
        match self {
            AspectType::Conjunction => 0.0,
            AspectType::SemiSextile => 30.0,
            AspectType::SemiSquare => 45.0,
            AspectType::Sextile => 60.0,
            AspectType::Quintile => 72.0,
            AspectType::Square => 90.0,
            AspectType::BiQuintile => 144.0,
            AspectType::Trine => 120.0,
            AspectType::Sesquisquare => 135.0,
            AspectType::Quincunx => 150.0,
            AspectType::Opposition => 180.0,
            AspectType::Septile => 51.428571,
            AspectType::BiSeptile => 102.857143,
            AspectType::TriSeptile => 154.285714,
            AspectType::Novile => 40.0,
            AspectType::BiNovile => 80.0,
            AspectType::QuadNovile => 160.0,
        }
    }

    pub fn orb(&self) -> f64 {
        match self {
            AspectType::Conjunction => 10.0,
            AspectType::SemiSextile => 3.0,
            AspectType::SemiSquare => 3.0,
            AspectType::Sextile => 8.0,
            AspectType::Quintile => 3.0,
            AspectType::Square => 10.0,
            AspectType::BiQuintile => 3.0,
            AspectType::Trine => 10.0,
            AspectType::Sesquisquare => 3.0,
            AspectType::Quincunx => 3.0,
            AspectType::Opposition => 10.0,
            AspectType::Septile => 2.0,
            AspectType::BiSeptile => 2.0,
            AspectType::TriSeptile => 2.0,
            AspectType::Novile => 2.0,
            AspectType::BiNovile => 2.0,
            AspectType::QuadNovile => 2.0,
        }
    }
}

/// Calculate aspects between planets
pub fn calculate_aspects(positions: &[PlanetPosition]) -> Vec<Aspect> {
    let mut aspects = Vec::new();
    
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let pos1 = &positions[i];
            let pos2 = &positions[j];
            
            // Skip if either planet is retrograde
            if pos1.is_retrograde || pos2.is_retrograde {
                continue;
            }
            
            let diff = (pos1.longitude - pos2.longitude).abs() % 360.0;
            let min_diff = diff.min(360.0 - diff);
            
            // Check each aspect type
            for aspect_type in [
                AspectType::Conjunction,
                AspectType::SemiSextile,
                AspectType::SemiSquare,
                AspectType::Sextile,
                AspectType::Quintile,
                AspectType::Square,
                AspectType::BiQuintile,
                AspectType::Trine,
                AspectType::Sesquisquare,
                AspectType::Quincunx,
                AspectType::Opposition,
                AspectType::Septile,
                AspectType::BiSeptile,
                AspectType::TriSeptile,
                AspectType::Novile,
                AspectType::BiNovile,
                AspectType::QuadNovile,
            ].iter() {
                let _aspect_angle = aspect_type.angle();
                let orb = aspect_type.orb();
                if (min_diff - _aspect_angle).abs() <= orb {
                    aspects.push(Aspect {
                        planet1: match i {
                            0 => "Sun".to_string(),
                            1 => "Moon".to_string(),
                            2 => "Mercury".to_string(),
                            3 => "Venus".to_string(),
                            4 => "Mars".to_string(),
                            5 => "Jupiter".to_string(),
                            6 => "Saturn".to_string(),
                            7 => "Uranus".to_string(),
                            8 => "Neptune".to_string(),
                            9 => "Pluto".to_string(),
                            _ => format!("Planet{}", i + 1),
                        },
                        planet2: match j {
                            0 => "Sun".to_string(),
                            1 => "Moon".to_string(),
                            2 => "Mercury".to_string(),
                            3 => "Venus".to_string(),
                            4 => "Mars".to_string(),
                            5 => "Jupiter".to_string(),
                            6 => "Saturn".to_string(),
                            7 => "Uranus".to_string(),
                            8 => "Neptune".to_string(),
                            9 => "Pluto".to_string(),
                            _ => format!("Planet{}", j + 1),
                        },
                        aspect_type: *aspect_type,
                        orb: (min_diff - _aspect_angle).abs(),
                    });
                }
            }
        }
    }
    
    aspects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_calculations() {
        let positions = vec![
            PlanetPosition {
                longitude: 0.0,
                latitude: 0.0,
                speed: 0.0,
                is_retrograde: false,
                house: Some(1),
            },
            PlanetPosition {
                longitude: 60.0,
                latitude: 0.0,
                speed: 1.0,
                is_retrograde: false,
                house: Some(2),
            },
        ];

        let aspects = calculate_aspects(&positions);
        println!("test_aspect_calculations: aspects = {:#?}", aspects);
        assert!(!aspects.is_empty());
        // Should find a sextile aspect
        let sextile = aspects.iter().find(|a| a.aspect_type == AspectType::Sextile);
        assert!(sextile.is_some());
        if let Some(sextile) = sextile {
            assert_eq!(sextile.planet1, "Sun");
            assert_eq!(sextile.planet2, "Moon");
            assert!(sextile.orb <= 8.0); // Sextile orb is 8°
        }
    }

    #[test]
    fn test_aspect_orbs() {
        let positions = vec![
            PlanetPosition {
                longitude: 0.0,
                latitude: 0.0,
                speed: 0.0,
                is_retrograde: false,
                house: Some(1),
            },
            PlanetPosition {
                longitude: 8.0,
                latitude: 0.0,
                speed: 1.0,
                is_retrograde: false,
                house: Some(2),
            },
        ];
        let aspects = calculate_aspects(&positions);
        assert!(!aspects.is_empty());
        // Should find a conjunction aspect
        let conjunction = aspects.iter().find(|a| a.aspect_type == AspectType::Conjunction);
        assert!(conjunction.is_some());
        if let Some(conjunction) = conjunction {
            assert_eq!(conjunction.planet1, "Sun");
            assert_eq!(conjunction.planet2, "Moon");
            assert!(conjunction.orb <= 10.0); // Conjunction orb is 10°
        }
    }

    #[test]
    fn test_retrograde_planets() {
        let positions = vec![
            PlanetPosition {
                longitude: 0.0,
                latitude: 0.0,
                speed: 0.0,
                is_retrograde: false,
                house: Some(1),
            },
            PlanetPosition {
                longitude: 60.0,
                latitude: 0.0,
                speed: 1.0,
                is_retrograde: true,
                house: Some(2),
            },
        ];
        let aspects = calculate_aspects(&positions);
        assert!(aspects.is_empty());
    }

    #[test]
    fn test_harmonic_aspects() {
        let positions = vec![
            PlanetPosition {
                longitude: 0.0,
                latitude: 0.0,
                speed: 0.0,
                is_retrograde: false,
                house: Some(1),
            },
            PlanetPosition {
                longitude: 72.0,
                latitude: 0.0,
                speed: 1.0,
                is_retrograde: false,
                house: Some(2),
            },
        ];
        let aspects = calculate_aspects(&positions);
        println!("test_harmonic_aspects: aspects = {:#?}", aspects);
        assert!(!aspects.is_empty());
        // Should find a quintile aspect
        let quintile = aspects.iter().find(|a| a.aspect_type == AspectType::Quintile);
        assert!(quintile.is_some());
        if let Some(quintile) = quintile {
            assert_eq!(quintile.planet1, "Sun");
            assert_eq!(quintile.planet2, "Moon");
            assert!(quintile.orb <= 3.0); // Quintile orb is 3°
        }
    }

    #[test]
    fn test_septile_aspects() {
        let positions = vec![
            PlanetPosition {
                longitude: 0.0,
                latitude: 0.0,
                speed: 0.0,
                is_retrograde: false,
                house: Some(1),
            },
            PlanetPosition {
                longitude: 51.428571,
                latitude: 0.0,
                speed: 1.0,
                is_retrograde: false,
                house: Some(2),
            },
        ];
        let aspects = calculate_aspects(&positions);
        println!("test_septile_aspects: aspects = {:#?}", aspects);
        assert!(!aspects.is_empty());
        // Should find a septile aspect
        let septile = aspects.iter().find(|a| a.aspect_type == AspectType::Septile);
        assert!(septile.is_some());
        if let Some(septile) = septile {
            assert_eq!(septile.planet1, "Sun");
            assert_eq!(septile.planet2, "Moon");
            assert!(septile.orb <= 2.0); // Septile orb is 2°
        }
    }

    #[test]
    fn test_novile_aspects() {
        let positions = vec![
            PlanetPosition {
                longitude: 0.0,
                latitude: 0.0,
                speed: 0.0,
                is_retrograde: false,
                house: Some(1),
            },
            PlanetPosition {
                longitude: 40.0,
                latitude: 0.0,
                speed: 1.0,
                is_retrograde: false,
                house: Some(2),
            },
        ];
        let aspects = calculate_aspects(&positions);
        println!("test_novile_aspects: aspects = {:#?}", aspects);
        assert!(!aspects.is_empty());
        // Should find a novile aspect
        let novile = aspects.iter().find(|a| a.aspect_type == AspectType::Novile);
        assert!(novile.is_some());
        if let Some(novile) = novile {
            assert_eq!(novile.planet1, "Sun");
            assert_eq!(novile.planet2, "Moon");
            assert!(novile.orb <= 2.0); // Novile orb is 2°
        }
    }
} 