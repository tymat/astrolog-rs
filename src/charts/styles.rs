use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::OnceLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref DEFAULT_STYLES: ChartStyles = ChartStyles::default();
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChartStyles {
    pub planet_colors: HashMap<String, String>,
    pub chart_colors: HashMap<String, String>,
    pub aspect_line_colors: HashMap<String, String>,
}

impl Default for ChartStyles {
    fn default() -> Self {
        Self {
            planet_colors: [
                ("Sun".to_string(), "#FF6B35".to_string()),
                ("Moon".to_string(), "#4ECDC4".to_string()),
                ("Mercury".to_string(), "#45B7D1".to_string()),
                ("Venus".to_string(), "#96CEB4".to_string()),
                ("Mars".to_string(), "#FFEAA7".to_string()),
                ("Jupiter".to_string(), "#DDA0DD".to_string()),
                ("Saturn".to_string(), "#98D8C8".to_string()),
                ("Uranus".to_string(), "#6C5CE7".to_string()),
                ("Neptune".to_string(), "#74B9FF".to_string()),
                ("Pluto".to_string(), "#A29BFE".to_string()),
            ].into_iter().collect(),
            chart_colors: [
                ("background".to_string(), "#FFFFFF".to_string()),
                ("wheel_background".to_string(), "#10002B".to_string()),
                ("chart_wheel_line".to_string(), "#9dade0".to_string()),
                ("chart1_planet_border".to_string(), "#252c42".to_string()),
                ("chart2_planet_border".to_string(), "#854077".to_string()),
                ("transit_planet_border".to_string(), "#8dad8c".to_string()),
                ("chart_text_color".to_string(), "#a1a4b3".to_string()),
                ("chart_aspect_color".to_string(), "#cbcfb4".to_string()),
            ].into_iter().collect(),
            aspect_line_colors: [
                ("Conjunction".to_string(), "#FF6B6B".to_string()),
                ("Opposition".to_string(), "#4ECDC4".to_string()),
                ("Trine".to_string(), "#45B7D1".to_string()),
                ("Square".to_string(), "#FFA07A".to_string()),
                ("Sextile".to_string(), "#98D8E8".to_string()),
            ].into_iter().collect(),
        }
    }
}

impl ChartStyles {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let styles: ChartStyles = serde_json::from_str(&content)?;
        Ok(styles)
    }

    pub fn get_planet_color(&self, planet: &str) -> &str {
        self.planet_colors.get(planet).map(|s| s.as_str()).unwrap_or("#333333")
    }

    pub fn get_chart_color(&self, color_key: &str) -> &str {
        self.chart_colors.get(color_key).map(|s| s.as_str()).unwrap_or("#333333")
    }

    pub fn get_aspect_color(&self, aspect: &str) -> &str {
        self.aspect_line_colors.get(aspect).map(|s| s.as_str()).unwrap_or("#666666")
    }
}

static GLOBAL_STYLES: OnceLock<ChartStyles> = OnceLock::new();

pub fn init_styles() -> Result<(), Box<dyn std::error::Error>> {
    let styles = match ChartStyles::load_from_file("chart_styles.json") {
        Ok(styles) => {
            log::info!("Loaded chart styles from chart_styles.json");
            styles
        }
        Err(e) => {
            log::warn!("Failed to load chart_styles.json: {}. Using default styles.", e);
            ChartStyles::default()
        }
    };
    
    GLOBAL_STYLES.set(styles).map_err(|_| "Failed to initialize global styles")?;
    Ok(())
}

pub fn get_styles() -> &'static ChartStyles {
    GLOBAL_STYLES.get().unwrap_or(&*DEFAULT_STYLES)
} 