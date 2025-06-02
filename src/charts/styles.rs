use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::{OnceLock, Once};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AspectLineColors {
    #[serde(default)]
    pub synastries: HashMap<String, String>,
    #[serde(default)]
    pub chart1: HashMap<String, String>,
    #[serde(default)]
    pub chart2: HashMap<String, String>,
    #[serde(flatten)]
    pub default_colors: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChartStyles {
    pub planet_colors: HashMap<String, String>,
    pub chart_colors: HashMap<String, String>,
    pub aspect_line_colors: AspectLineColors,
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
        self.aspect_line_colors.default_colors.get(aspect).map(|s| s.as_str()).unwrap_or("#666666")
    }

    pub fn get_chart1_aspect_color(&self, aspect: &str) -> &str {
        self.aspect_line_colors.chart1.get(aspect)
            .map(|s| s.as_str())
            .or_else(|| self.aspect_line_colors.default_colors.get(aspect).map(|s| s.as_str()))
            .unwrap_or("#666666")
    }

    pub fn get_chart2_aspect_color(&self, aspect: &str) -> &str {
        self.aspect_line_colors.chart2.get(aspect)
            .map(|s| s.as_str())
            .or_else(|| self.aspect_line_colors.default_colors.get(aspect).map(|s| s.as_str()))
            .unwrap_or("#666666")
    }

    pub fn get_synastry_aspect_color(&self, aspect: &str) -> &str {
        self.aspect_line_colors.synastries.get(aspect)
            .map(|s| s.as_str())
            .or_else(|| self.aspect_line_colors.default_colors.get(aspect).map(|s| s.as_str()))
            .unwrap_or("#666666")
    }
}

static GLOBAL_STYLES: OnceLock<ChartStyles> = OnceLock::new();
static INIT_ONCE: Once = Once::new();

fn try_load_styles() -> Result<ChartStyles, Box<dyn std::error::Error>> {
    // Try multiple possible paths for the chart styles files
    // Prioritize the new format, then fall back to the old format
    let possible_paths = vec![
        "chart_styles_new.json".to_string(),                               // New format - current directory
        "./chart_styles_new.json".to_string(),                             // New format - explicit current directory
        format!("{}/chart_styles_new.json", env!("CARGO_MANIFEST_DIR")),   // New format - relative to Cargo.toml
        "chart_styles.json".to_string(),                                    // Old format - current directory
        "./chart_styles.json".to_string(),                                  // Old format - explicit current directory
        "astrolog-rs/chart_styles.json".to_string(),                       // Old format - from parent directory
        "../chart_styles.json".to_string(),                                // Old format - parent directory
        format!("{}/chart_styles.json", env!("CARGO_MANIFEST_DIR")),       // Old format - relative to Cargo.toml
    ];
    
    let mut last_error = None;
    
    for path in &possible_paths {
        match ChartStyles::load_from_file(path) {
            Ok(loaded_styles) => {
                log::info!("Loaded chart styles from {}", path);
                return Ok(loaded_styles);
            }
            Err(e) => {
                log::debug!("Failed to load chart styles from {}: {}", path, e);
                last_error = Some(e);
            }
        }
    }
    
    // If we get here, no file was found - this is an error
    let error_msg = format!(
        "Failed to load chart styles from any location. Tried: {}. Last error: {}",
        possible_paths.join(", "),
        last_error.map(|e| e.to_string()).unwrap_or_else(|| "Unknown error".to_string())
    );
    
    Err(error_msg.into())
}

pub fn init_styles() -> Result<(), Box<dyn std::error::Error>> {
    try_load_styles().map(|styles| {
        let _ = GLOBAL_STYLES.set(styles);
    })
}

pub fn get_styles() -> Option<&'static ChartStyles> {
    // Try to get existing styles first
    if let Some(styles) = GLOBAL_STYLES.get() {
        return Some(styles);
    }
    
    // If not initialized, try to initialize once
    let mut init_result = Ok(());
    INIT_ONCE.call_once(|| {
        init_result = try_load_styles().map(|styles| {
            let _ = GLOBAL_STYLES.set(styles);
        });
    });
    
    // Return styles if available, regardless of initialization result
    GLOBAL_STYLES.get()
} 