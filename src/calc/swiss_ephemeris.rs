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

/// Swiss Ephemeris planet constants.
/// These constants are used to identify celestial bodies in the Swiss Ephemeris calculations.
#[allow(dead_code)]
pub const SE_SUN: i32 = 0;      /// The Sun
#[allow(dead_code)]
pub const SE_MOON: i32 = 1;     /// The Moon
#[allow(dead_code)]
pub const SE_MERCURY: i32 = 2;  /// Mercury
#[allow(dead_code)]
pub const SE_VENUS: i32 = 3;    /// Venus
#[allow(dead_code)]
pub const SE_MARS: i32 = 4;     /// Mars
#[allow(dead_code)]
pub const SE_JUPITER: i32 = 5;  /// Jupiter
#[allow(dead_code)]
pub const SE_SATURN: i32 = 6;   /// Saturn
#[allow(dead_code)]
pub const SE_URANUS: i32 = 7;   /// Uranus
#[allow(dead_code)]
pub const SE_NEPTUNE: i32 = 8;  /// Neptune
#[allow(dead_code)]
pub const SE_PLUTO: i32 = 9;    /// Pluto
#[allow(dead_code)]
pub const SE_MEAN_NODE: i32 = 10;  /// Mean Lunar Node
#[allow(dead_code)]
pub const SE_TRUE_NODE: i32 = 11;   /// True Lunar Node
#[allow(dead_code)]
pub const SE_CHIRON: i32 = 15;      /// Chiron
#[allow(dead_code)]
pub const SE_MEAN_APOG: i32 = 20;   /// Mean Lunar Apogee
#[allow(dead_code)]
pub const SE_OSCU_APOG: i32 = 21;   /// Osculating Lunar Apogee
#[allow(dead_code)]
pub const SE_EARTH: i32 = 14;       /// Earth
#[allow(dead_code)]
pub const SE_ASC: i32 = 0;          /// Ascendant
#[allow(dead_code)]
pub const SE_MC: i32 = 1;           /// Midheaven
#[allow(dead_code)]
pub const SE_ARMC: i32 = 2;         /// Armc (Apparent Right Ascension of Meridian)
#[allow(dead_code)]
pub const SE_VERTEX: i32 = 3;       /// Vertex
#[allow(dead_code)]
pub const SE_EQUASC: i32 = 4;       /// Equatorial Ascendant
#[allow(dead_code)]
pub const SE_COASC1: i32 = 5;       /// Co-Ascendant 1
#[allow(dead_code)]
pub const SE_COASC2: i32 = 6;       /// Co-Ascendant 2
#[allow(dead_code)]
pub const SE_POLASC: i32 = 7;       /// Polar Ascendant
#[allow(dead_code)]
pub const SE_NASCMC: i32 = 8;       /// Non-Ascending Midheaven

/// Initializes the Swiss Ephemeris library.
///
/// This function must be called before using any Swiss Ephemeris functions.
/// It sets up the ephemeris files and initializes the library.
///
/// # Returns
///
/// A Result indicating success or failure of initialization
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::swiss_ephemeris::init_swiss_ephemeris;
///
/// match init_swiss_ephemeris() {
///     Ok(_) => println!("Swiss Ephemeris initialized successfully"),
///     Err(e) => println!("Failed to initialize Swiss Ephemeris: {}", e),
/// }
/// ```
#[allow(dead_code)]
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

/// Calculates the position of a planet using the Swiss Ephemeris.
///
/// This function calculates the geocentric position of a planet at a given time.
/// The Swiss Ephemeris provides high-precision planetary positions.
///
/// # Arguments
///
/// * `planet` - The Swiss Ephemeris planet number
/// * `year` - The year
/// * `month` - The month (1-12)
/// * `day` - The day (1-31)
/// * `hour` - The hour (0-23)
///
/// # Returns
///
/// A Result containing a tuple with:
/// * Longitude in degrees (0-360)
/// * Latitude in degrees (-90 to 90)
/// * Distance in AU
/// * Speed in degrees per day
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::swiss_ephemeris::calculate_planet_position_swiss;
/// use swisseph::Planet;
///
/// match calculate_planet_position_swiss(Planet::Sun, 2000, 1, 1, 12.0) {
///     Ok((longitude, latitude, distance, speed)) => {
///         println!("Sun position: {}° longitude, {}° latitude", longitude, latitude);
///         println!("Distance: {} AU, Speed: {}°/day", distance, speed);
///     },
///     Err(e) => println!("Error calculating planet position: {}", e),
/// }
/// ```
pub fn calculate_planet_position_swiss(
    planet: SwePlanet,
    year: i32,
    month: i32,
    day: i32,
    hour: f64,
) -> Result<(f64, f64, f64, f64), AstrologError> {
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
    let speed = pos[3];

    Ok((longitude, latitude, distance, speed))
}

/// Maps an astrolog Planet enum to a Swiss Ephemeris planet number.
///
/// This function converts between the astrolog library's Planet enum and
/// the Swiss Ephemeris planet numbers.
///
/// # Arguments
///
/// * `planet` - The astrolog Planet enum value
///
/// # Returns
///
/// An Option containing the Swiss Ephemeris planet number if the mapping exists
///
/// # Examples
///
/// ```
/// use astrolog_rs::calc::planets::Planet;
/// use astrolog_rs::calc::swiss_ephemeris::map_planet_to_swe;
///
/// let planet = Planet::Sun;
/// match map_planet_to_swe(planet) {
///     Some(swe_planet) => println!("Swiss Ephemeris planet number: {:?}", swe_planet),
///     None => println!("No mapping found for this planet"),
/// }
/// ```
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

/// Calculates house cusps using the Swiss Ephemeris.
///
/// This function calculates the positions of the house cusps using the
/// Swiss Ephemeris library, which provides high-precision calculations.
///
/// # Arguments
///
/// * `julian_date` - The Julian date for the calculation
/// * `latitude` - The geographical latitude in degrees (-90 to 90)
/// * `longitude` - The geographical longitude in degrees (-180 to 180)
/// * `house_system` - The house system to use
///
/// # Returns
///
/// A Result containing a tuple with:
/// * A vector of 13 house cusp positions (0-12) in degrees (0-360)
/// * A tuple of (Ascendant, MC) positions in degrees
///
/// # Examples
///
/// ```
/// use astrolog_rs::core::types::HouseSystem;
/// use astrolog_rs::calc::swiss_ephemeris::calculate_house_cusps_swiss;
///
/// let julian_date = 2451545.0; // 2000-01-01
/// let latitude = 40.0;
/// let longitude = -74.0;
///
/// match calculate_house_cusps_swiss(julian_date, latitude, longitude, HouseSystem::Placidus) {
///     Ok((cusps, ascmc)) => {
///         println!("Ascendant: {}°, MC: {}°", ascmc[0], ascmc[1]);
///         for (i, cusp) in cusps.iter().enumerate() {
///             println!("House {} cusp: {}°", i, cusp);
///         }
///     },
///     Err(e) => println!("Error calculating house cusps: {}", e),
/// }
/// ```
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
