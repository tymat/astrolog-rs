use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Utc};

/// Maximum number of objects that can be tracked
pub const OBJ_MAX: usize = 100;

/// Number of zodiac signs
pub const SIGN_COUNT: usize = 12;

/// Basic chart information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartInfo {
    pub name: String,
    pub date: DateTime<Utc>,
    pub timezone: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: HouseSystem,
}

/// Chart positions for all objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPositions {
    pub sun: Position,
    pub moon: Position,
    pub mercury: Position,
    pub venus: Position,
    pub mars: Position,
    pub jupiter: Position,
    pub saturn: Position,
    pub uranus: Position,
    pub neptune: Position,
    pub pluto: Position,
    pub mean_node: Position,
    pub true_node: Position,
    pub mean_lilith: Position,
    pub osc_lilith: Position,
    pub chiron: Position,
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

/// Error types for the application
#[derive(Debug)]
pub enum AstrologError {
    InvalidDate,
    InvalidTime,
    InvalidLocation,
    CalculationError,
    IOError,
    NotImplemented(String),
}

impl fmt::Display for AstrologError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstrologError::InvalidDate => write!(f, "Invalid date"),
            AstrologError::InvalidTime => write!(f, "Invalid time"),
            AstrologError::InvalidLocation => write!(f, "Invalid location"),
            AstrologError::CalculationError => write!(f, "Calculation error"),
            AstrologError::IOError => write!(f, "IO error"),
            AstrologError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for AstrologError {}

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HouseSystem {
    Placidus = 0,
    Koch = 1,
    Equal = 2,
    WholeSign = 3,
    Campanus = 4,
    Regiomontanus = 5,
    Meridian = 6,
    Alcabitius = 7,
    Morinus = 8,
    Krusinski = 9,
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
            "morinus" => Ok(HouseSystem::Morinus),
            "krusinski" => Ok(HouseSystem::Krusinski),
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