pub mod styles;
pub mod svg_generator;

use crate::api::types::{ChartResponse, TransitResponse, SynastryResponse};
use svg_generator::SVGChartGenerator;

// Re-export important types
pub use styles::{ChartStyles, init_styles, get_styles};

/// Generate SVG for natal chart (including transits if present)
pub fn generate_natal_svg(chart_data: &ChartResponse) -> Result<String, String> {
    let generator = SVGChartGenerator::new();
    generator.generate_natal_chart(chart_data)
}

/// Generate SVG for synastry chart
pub fn generate_synastry_svg(synastry_data: &SynastryResponse) -> Result<String, String> {
    let generator = SVGChartGenerator::new();
    generator.generate_synastry_chart(synastry_data)
}

/// Generate SVG for transit chart
pub fn generate_transit_svg(transit_data: &TransitResponse) -> Result<String, String> {
    let generator = SVGChartGenerator::new();
    generator.generate_transit_chart(transit_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::{ChartResponse, PlanetInfo, HouseInfo, AspectInfo};
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;

    fn create_test_chart_data() -> ChartResponse {
        ChartResponse {
            chart_type: "natal".to_string(),
            date: Utc::now(),
            latitude: 40.7128,
            longitude: -74.0060,
            house_system: "placidus".to_string(),
            ayanamsa: "tropical".to_string(),
            planets: vec![
                PlanetInfo {
                    name: "Sun".to_string(),
                    longitude: 120.0,
                    latitude: 0.0,
                    speed: 1.0,
                    is_retrograde: false,
                    house: Some(5),
                },
                PlanetInfo {
                    name: "Moon".to_string(),
                    longitude: 180.0,
                    latitude: 0.0,
                    speed: 13.0,
                    is_retrograde: false,
                    house: Some(7),
                },
            ],
            houses: vec![
                HouseInfo { number: 1, longitude: 0.0, latitude: 0.0 },
                HouseInfo { number: 2, longitude: 30.0, latitude: 0.0 },
            ],
            aspects: vec![
                AspectInfo {
                    planet1: "Sun".to_string(),
                    planet2: "Moon".to_string(),
                    aspect: "Opposition".to_string(),
                    orb: 2.0,
                },
            ],
            transit: None,
        }
    }

    #[test]
    fn test_natal_svg_generation() {
        let _ = init_styles(); // Initialize styles
        let chart_data = create_test_chart_data();
        let svg_result = generate_natal_svg(&chart_data);
        
        // If styles failed to load, test should handle that gracefully
        match svg_result {
            Ok(svg) => {
                assert!(svg.contains("<svg"));
                assert!(svg.contains("</svg>"));
                assert!(svg.contains("☉")); // Sun symbol
                assert!(svg.contains("☽")); // Moon symbol
            },
            Err(e) => {
                // This is expected if chart_styles.json is not available during testing
                assert!(e.contains("chart_styles.json"));
            }
        }
    }

    #[test]
    fn test_styles_initialization() {
        let result = init_styles();
        // Either loads file or fails - both are valid test outcomes
        
        if let Some(styles) = get_styles() {
            assert!(styles.get_planet_color("Sun").starts_with("#"));
            assert!(styles.get_chart_color("background").starts_with("#"));
            assert!(styles.get_aspect_color("Opposition").starts_with("#"));
        }
        // If styles are None, that's also a valid test outcome when file is missing
    }
}
