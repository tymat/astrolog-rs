use super::super::aspects::*;

#[test]
fn test_get_aspect_angle() {
    assert_eq!(get_aspect_angle(AspectType::Conjunction), 0.0);
    assert_eq!(get_aspect_angle(AspectType::Opposition), 180.0);
    assert_eq!(get_aspect_angle(AspectType::Trine), 120.0);
    assert_eq!(get_aspect_angle(AspectType::Square), 90.0);
    assert_eq!(get_aspect_angle(AspectType::Sextile), 60.0);
}

#[test]
fn test_calculate_aspect() {
    // Test exact conjunction
    let aspect = calculate_aspect(0.0, 0.0, AspectType::Conjunction, 10.0);
    assert!(aspect.is_some());
    let config = aspect.unwrap();
    assert_eq!(config.orb, 0.0);
    assert!(!config.applying);

    // Test conjunction within orb
    let aspect = calculate_aspect(5.0, 0.0, AspectType::Conjunction, 10.0);
    assert!(aspect.is_some());
    let config = aspect.unwrap();
    assert_eq!(config.orb, 5.0);
    assert!(config.applying);

    // Test conjunction outside orb
    let aspect = calculate_aspect(15.0, 0.0, AspectType::Conjunction, 10.0);
    assert!(aspect.is_none());

    // Test exact opposition
    let aspect = calculate_aspect(0.0, 180.0, AspectType::Opposition, 10.0);
    assert!(aspect.is_some());
    let config = aspect.unwrap();
    assert_eq!(config.orb, 0.0);
    assert!(!config.applying);
}

#[test]
fn test_calculate_all_aspects() {
    let positions = vec![0.0, 60.0, 90.0, 120.0, 180.0];
    let orbs = vec![10.0, 10.0, 10.0, 10.0, 10.0];
    let aspect_types = vec![
        AspectType::Conjunction,
        AspectType::Sextile,
        AspectType::Square,
        AspectType::Trine,
        AspectType::Opposition,
    ];

    let aspects = calculate_all_aspects(&positions, &orbs, &aspect_types);
    
    // Should find at least these aspects:
    // - 0° and 60° (sextile)
    // - 0° and 90° (square)
    // - 0° and 120° (trine)
    // - 0° and 180° (opposition)
    assert!(aspects.len() >= 4);
}

#[test]
fn test_calculate_aspect_time() {
    // Test aspect time calculation
    let time = calculate_aspect_time(0.0, 1.0, 90.0, 0.5, AspectType::Square);
    assert!(time.is_some());
    let t = time.unwrap();
    assert!(t > 0.0);

    // Test when planets are moving away from aspect
    let time = calculate_aspect_time(0.0, 0.5, 90.0, 1.0, AspectType::Square);
    assert!(time.is_none());

    // Test when planets are moving at same speed
    let time = calculate_aspect_time(0.0, 1.0, 90.0, 1.0, AspectType::Square);
    assert!(time.is_none());
}

#[test]
fn test_is_aspect_applying() {
    // Test applying conjunction
    assert!(is_aspect_applying(5.0, 0.0, AspectType::Conjunction));
    
    // Test separating conjunction
    assert!(!is_aspect_applying(355.0, 0.0, AspectType::Conjunction));
    
    // Test applying opposition
    assert!(is_aspect_applying(175.0, 0.0, AspectType::Opposition));
    
    // Test separating opposition
    assert!(!is_aspect_applying(185.0, 0.0, AspectType::Opposition));
} 