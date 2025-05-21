use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Utc};

/// Maximum number of objects that can be tracked
pub const OBJ_MAX: usize = 100;

/// Number of zodiac signs
pub const SIGN_COUNT: usize = 12;

/// Represents errors that can occur during astrological calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstrologError {
    /// Error during calculation of planetary positions
    CalculationError {
        message: String,
    },
    /// Error during house system calculations
    HouseSystemError {
        message: String,
        system: String,
    },
    /// Error during coordinate transformations
    CoordinateError {
        message: String,
        from: String,
        to: String,
    },
    /// Error during aspect calculations
    AspectError {
        message: String,
        planets: (String, String),
    },
    /// Error during date/time calculations
    DateTimeError {
        message: String,
        date: Option<DateTime<Utc>>,
    },
    /// Error during location-based calculations
    LocationError {
        message: String,
        latitude: Option<f64>,
        longitude: Option<f64>,
    },
    /// Error for unimplemented features
    NotImplemented {
        message: String,
    },
    /// Error for invalid input parameters
    InvalidInput {
        message: String,
        parameter: String,
    },
    /// Error for invalid latitude
    InvalidLatitude(String),
}

impl fmt::Display for AstrologError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstrologError::CalculationError { message } => {
                write!(f, "Calculation error: {}", message)
            }
            AstrologError::HouseSystemError { message, system } => {
                write!(f, "House system error ({}): {}", system, message)
            }
            AstrologError::CoordinateError { message, from, to } => {
                write!(f, "Coordinate transformation error ({} to {}): {}", from, to, message)
            }
            AstrologError::AspectError { message, planets } => {
                write!(f, "Aspect error between {} and {}: {}", planets.0, planets.1, message)
            }
            AstrologError::DateTimeError { message, date } => {
                if let Some(dt) = date {
                    write!(f, "Date/time error at {}: {}", dt, message)
                } else {
                    write!(f, "Date/time error: {}", message)
                }
            }
            AstrologError::LocationError { message, latitude, longitude } => {
                if let (Some(lat), Some(lon)) = (latitude, longitude) {
                    write!(f, "Location error at ({}, {}): {}", lat, lon, message)
                } else {
                    write!(f, "Location error: {}", message)
                }
            }
            AstrologError::NotImplemented { message } => {
                write!(f, "Not implemented: {}", message)
            }
            AstrologError::InvalidInput { message, parameter } => {
                write!(f, "Invalid input for {}: {}", parameter, message)
            }
            AstrologError::InvalidLatitude(message) => {
                write!(f, "Invalid latitude: {}", message)
            }
        }
    }
}

impl std::error::Error for AstrologError {}

/// Information about a chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartInfo {
    pub julian_date: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
}

/// Positions of celestial bodies in a chart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPositions {
    pub zodiac_positions: Vec<f64>,
    pub house_cusps: Vec<f64>,
    pub house_placements: Vec<u8>,
}

impl ChartPositions {
    pub fn new() -> Self {
        Self {
            zodiac_positions: Vec::new(),
            house_cusps: Vec::new(),
            house_placements: Vec::new(),
        }
    }
}

/// User settings for chart generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    // Chart types
    pub listing: bool,
    pub wheel: bool,
    pub grid: bool,
    pub aspect_list: bool,
    pub midpoint: bool,
    pub horizon: bool,
    pub orbit: bool,
    pub sector: bool,
    pub influence: bool,
    pub astro_graph: bool,
    pub calendar: bool,
    pub in_day: bool,
    pub ephemeris: bool,
    pub transit: bool,

    // Chart options
    pub sidereal: bool,
    pub cusp: bool,
    pub uranian: bool,
    pub progress: bool,
    pub interpret: bool,
    pub decan: bool,
    pub flip: bool,
    pub geodetic: bool,
    pub vedic: bool,
    pub navamsa: bool,
    pub placalc: bool,
    pub write_file: bool,
    pub ansi_color: bool,
    pub graphics: bool,

    // Value settings
    pub house_system: u8,
    pub aspect_count: u8,
    pub center_object: u8,
    pub star_count: u8,
    pub harmonic: u8,
    pub object_on_asc: u8,
    pub day_delta: i32,
    pub degree_format: u8,
    pub division: u8,
    pub screen_width: u16,
    pub dst_default: f64,
    pub zone_default: f64,
    pub longitude_default: f64,
    pub latitude_default: f64,
}

/// Graphics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub bitmap: bool,
    pub postscript: bool,
    pub metafile: bool,
    pub color: bool,
    pub inverse: bool,
    pub root: bool,
    pub text: bool,
    pub font: bool,
    pub alt: bool,
    pub border: bool,
    pub label: bool,
    pub jet_trail: bool,
    pub mouse: bool,
    pub constellation: bool,
    pub mollewide: bool,
    pub print_map: bool,
    pub window_width: i32,
    pub window_height: i32,
    pub animation_mode: i32,
    pub scale: i32,
    pub left_object: i32,
    pub text_rows: i32,
    pub rotation: i32,
    pub tilt: f64,
    pub bitmap_mode: char,
    pub orientation: i32,
    pub paper_width: f64,
    pub paper_height: f64,
    pub display: Option<String>,
    pub grid_cells: i32,
    pub glyphs: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aspect {
    pub planet1: String,
    pub planet2: String,
    pub aspect_type: String,
    pub orb: f64,
    pub applying: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub info: ChartInfo,
    pub positions: ChartPositions,
    pub houses: [f64; 12],
    pub aspects: Vec<Aspect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
    pub retrograde: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HouseSystem {
    Placidus,
    Koch,
    Equal,
    WholeSign,
    Campanus,
    Regiomontanus,
    Meridian,
    Alcabitius,
    Topocentric,
    Morinus,
    Porphyrius,
    Krusinski,
    Vedic,
    Null,
}

impl std::fmt::Display for HouseSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HouseSystem::Placidus => write!(f, "Placidus"),
            HouseSystem::Koch => write!(f, "Koch"),
            HouseSystem::Equal => write!(f, "Equal"),
            HouseSystem::WholeSign => write!(f, "Whole Sign"),
            HouseSystem::Campanus => write!(f, "Campanus"),
            HouseSystem::Regiomontanus => write!(f, "Regiomontanus"),
            HouseSystem::Meridian => write!(f, "Meridian"),
            HouseSystem::Alcabitius => write!(f, "Alcabitius"),
            HouseSystem::Topocentric => write!(f, "Topocentric"),
            HouseSystem::Morinus => write!(f, "Morinus"),
            HouseSystem::Porphyrius => write!(f, "Porphyrius"),
            HouseSystem::Krusinski => write!(f, "Krusinski"),
            HouseSystem::Vedic => write!(f, "Vedic"),
            HouseSystem::Null => write!(f, "Null"),
        }
    }
}

impl FromStr for HouseSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "placidus" => Ok(HouseSystem::Placidus),
            "koch" => Ok(HouseSystem::Koch),
            "equal" => Ok(HouseSystem::Equal),
            "wholesign" => Ok(HouseSystem::WholeSign),
            "campanus" => Ok(HouseSystem::Campanus),
            "regiomontanus" => Ok(HouseSystem::Regiomontanus),
            "meridian" => Ok(HouseSystem::Meridian),
            "alcabitius" => Ok(HouseSystem::Alcabitius),
            "topocentric" => Ok(HouseSystem::Topocentric),
            "morinus" => Ok(HouseSystem::Morinus),
            "porphyrius" => Ok(HouseSystem::Porphyrius),
            "krusinski" => Ok(HouseSystem::Krusinski),
            "vedic" => Ok(HouseSystem::Vedic),
            "null" => Ok(HouseSystem::Null),
            _ => Err(format!("Invalid house system: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AspectType {
    Conjunction = 0,
    Opposition = 1,
    Trine = 2,
    Square = 3,
    Sextile = 4,
    Semisextile = 5,
    Semisquare = 6,
    Sesquisquare = 7,
    Quintile = 8,
    Biquintile = 9,
    Quincunx = 10,
} 