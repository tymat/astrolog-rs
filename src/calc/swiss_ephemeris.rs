use crate::calc::swiss_ephemeris_ffi;
use crate::core::types::AstrologError;
use crate::core::types::HouseSystem;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::sync::Once;
use swisseph::{self, Planet as SwePlanet};

// Use a local path for ephemeris files
const EPHE_PATH: &str = "./ephe";

// Global initialization flag
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// Global Swisseph instance
static SWISSEPH: Mutex<Option<swisseph::Swisseph>> = Mutex::new(None);

// One-time initialization
static INIT: Once = Once::new();

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
    // Only initialize once
    INIT.call_once(|| {
        // Create the ephemeris directory if it doesn't exist
        let ephe_path = PathBuf::from(EPHE_PATH);
        if let Err(e) = std::fs::create_dir_all(&ephe_path) {
            eprintln!("Failed to create ephemeris directory: {}", e);
            return;
        }

        // Check if required ephemeris files exist
        let required_files = ["seas_18.se1", "semo_18.se1", "sepl_18.se1"];
        let missing_files: Vec<String> = required_files
            .iter()
            .filter(|&&file| !ephe_path.join(file).exists())
            .map(|&s| s.to_string())
            .collect();

        if !missing_files.is_empty() {
            eprintln!(
                "Missing required ephemeris files: {}. Please download the Swiss Ephemeris package from https://www.astro.com/swisseph/ and place the files in the {} directory.",
                missing_files.join(", "),
                EPHE_PATH
            );
            return;
        }

        // Create a new Swisseph instance and set the path
        let mut swe = swisseph::Swisseph::new();
        swe.set_ephe_path(swisseph::EphePath::from(EPHE_PATH));

        // Store the instance
        if let Ok(mut guard) = SWISSEPH.lock() {
            *guard = Some(swe);
            INITIALIZED.store(true, Ordering::SeqCst);
        }
    });

    if !INITIALIZED.load(Ordering::SeqCst) {
        return Err(AstrologError::CalculationError {
            message: "Failed to initialize Swiss Ephemeris".to_string(),
        });
    }

    Ok(())
}

pub fn calculate_planet_position_swiss(
    planet: SwePlanet,
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
) -> Result<(f64, f64, f64), AstrologError> {
    if !INITIALIZED.load(Ordering::SeqCst) {
        return Err(AstrologError::CalculationError {
            message: "Swiss Ephemeris not initialized".to_string(),
        });
    }

    let guard = SWISSEPH
        .lock()
        .map_err(|_| AstrologError::CalculationError {
            message: "Failed to acquire Swiss Ephemeris lock".to_string(),
        })?;

    let swe = guard
        .as_ref()
        .ok_or_else(|| AstrologError::CalculationError {
            message: "Swiss Ephemeris instance not available".to_string(),
        })?;

    let jd = swe.julday(year, month, day, hour, true); // true = Gregorian

    // Use default flags for geocentric positions
    let flags = swisseph::Flags::default();
    let pos = swe
        .calc_ut(jd, planet, flags)
        .map_err(|e| AstrologError::CalculationError {
            message: format!("Swiss Ephemeris error: {e}"),
        })?;

    // Convert to zodiacal longitude (0-360 degrees)
    let longitude = pos[0].rem_euclid(360.0);
    let latitude = pos[1];
    let distance = pos[2];

    Ok((longitude, latitude, distance))
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

pub fn calculate_house_cusps_swiss(
    jd_ut: f64,
    geolat: f64,
    geolon: f64,
    house_system: HouseSystem,
) -> Result<([f64; 13], [f64; 10]), AstrologError> {
    let mut cusps = [0.0f64; 13];
    let mut ascmc = [0.0f64; 10];

    // Map our house systems to Swiss Ephemeris codes
    let hsys = match house_system {
        HouseSystem::Placidus => b'P',
        HouseSystem::Koch => b'K',
        HouseSystem::Equal => b'A',
        HouseSystem::WholeSign => b'W',
        HouseSystem::Campanus => b'C',
        HouseSystem::Regiomontanus => b'R',
        HouseSystem::Meridian => b'X',
        HouseSystem::Alcabitius => b'B',
        HouseSystem::Topocentric => b'T',
        HouseSystem::Morinus => b'M',
        HouseSystem::Porphyrius => b'O',
        HouseSystem::Krusinski => b'U',
        HouseSystem::Vedic => b'W', // Use whole sign for Vedic
        HouseSystem::Null => b'A',  // Use equal for Null
    };

    let ret = unsafe {
        swiss_ephemeris_ffi::swe_houses(
            jd_ut,
            geolat,
            geolon,
            hsys as i32,
            cusps.as_mut_ptr(),
            ascmc.as_mut_ptr(),
        )
    };
    if ret < 0 {
        return Err(AstrologError::CalculationError {
            message: "Swiss Ephemeris swe_houses failed".to_string(),
        });
    }
    Ok((cusps, ascmc))
}
