use swisseph::{self, Planet as SwePlanet};
use crate::core::types::AstrologError;
use std::sync::OnceLock;
use std::path::PathBuf;

// Use a local path for ephemeris files
const EPHE_PATH: &str = "./ephe";

// Singleton Swisseph instance
static SWISSEPH: OnceLock<swisseph::Swisseph> = OnceLock::new();

// Swiss Ephemeris planet constants
pub const SE_SUN: i32 = 0;
pub const SE_MOON: i32 = 1;
pub const SE_MERCURY: i32 = 2;
pub const SE_VENUS: i32 = 3;
pub const SE_MARS: i32 = 4;
pub const SE_JUPITER: i32 = 5;
pub const SE_SATURN: i32 = 6;
pub const SE_URANUS: i32 = 7;
pub const SE_NEPTUNE: i32 = 8;
pub const SE_PLUTO: i32 = 9;
pub const SE_MEAN_NODE: i32 = 10;
pub const SE_TRUE_NODE: i32 = 11;

pub fn init_swiss_ephemeris() -> Result<(), AstrologError> {
    // Create the ephemeris directory if it doesn't exist
    let ephe_path = PathBuf::from(EPHE_PATH);
    std::fs::create_dir_all(&ephe_path).map_err(|e| AstrologError::CalculationError {
        message: format!("Failed to create ephemeris directory: {}", e),
    })?;

    // Check if required ephemeris files exist
    let required_files = ["seas_18.se1", "semo_18.se1", "sepl_18.se1"];
    let missing_files: Vec<String> = required_files
        .iter()
        .filter(|&&file| !ephe_path.join(file).exists())
        .map(|&s| s.to_string())
        .collect();

    if !missing_files.is_empty() {
        return Err(AstrologError::CalculationError {
            message: format!(
                "Missing required ephemeris files: {}. Please download the Swiss Ephemeris package from https://www.astro.com/swisseph/ and place the files in the {} directory.",
                missing_files.join(", "),
                EPHE_PATH
            ),
        });
    }

    let mut swe = swisseph::Swisseph::new();
    swe.set_ephe_path(swisseph::EphePath::from(EPHE_PATH));
    SWISSEPH.set(swe).map_err(|_| AstrologError::CalculationError {
        message: "Swiss Ephemeris already initialized".to_string(),
    })?;
    Ok(())
}

pub fn calculate_planet_position_swiss(
    planet: SwePlanet,
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
) -> Result<(f64, f64, f64), AstrologError> {
    let swe = SWISSEPH.get().ok_or_else(|| AstrologError::CalculationError {
        message: "Swiss Ephemeris not initialized".to_string(),
    })?;
    
    let jd = swe.julday(year, month, day, hour, true); // true = Gregorian
    let pos = swe.calc_ut(jd, planet, swisseph::Flags::default())
        .map_err(|e| AstrologError::CalculationError { message: format!("Swiss Ephemeris error: {e}") })?;
    
    // pos[0] = longitude, pos[1] = latitude, pos[2] = distance
    Ok((pos[0], pos[1], pos[2]))
}

// Map our Planet enum to swisseph::Planet
pub fn map_planet_to_swe(planet: crate::calc::planets::Planet) -> Option<SwePlanet> {
    match planet {
        crate::calc::planets::Planet::Sun => Some(SwePlanet::Sun),
        crate::calc::planets::Planet::Moon => Some(SwePlanet::Moon),
        crate::calc::planets::Planet::Mercury => Some(SwePlanet::Mercury),
        crate::calc::planets::Planet::Venus => Some(SwePlanet::Venus),
        crate::calc::planets::Planet::Mars => Some(SwePlanet::Mars),
        crate::calc::planets::Planet::Jupiter => Some(SwePlanet::Jupiter),
        crate::calc::planets::Planet::Saturn => Some(SwePlanet::Saturn),
        crate::calc::planets::Planet::Uranus => Some(SwePlanet::Uranus),
        crate::calc::planets::Planet::Neptune => Some(SwePlanet::Neptune),
        crate::calc::planets::Planet::Pluto => Some(SwePlanet::Pluto),
        crate::calc::planets::Planet::MeanNode => Some(SwePlanet::MeanNode),
        crate::calc::planets::Planet::TrueNode => Some(SwePlanet::TrueNode),
        _ => None,
    }
} 