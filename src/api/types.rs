use crate::calc::planets::PlanetPosition;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransitInfo {
    pub date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for TransitInfo {
    fn default() -> Self {
        Self {
            date: Utc::now(),
            latitude: 51.45,  // London coordinates as default
            longitude: 0.05,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartRequest {
    pub date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
    pub ayanamsa: String,
    #[serde(default)]
    pub transit: Option<TransitInfo>,
    #[serde(default)]
    pub include_minor_aspects: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransitRequest {
    pub natal_date: DateTime<Utc>,
    pub transit_date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
    pub ayanamsa: String,
    #[serde(default)]
    pub include_minor_aspects: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SynastryRequest {
    pub chart1: ChartRequest,
    pub chart2: ChartRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanetInfo {
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub speed: f64,
    pub is_retrograde: bool,
    pub house: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HouseInfo {
    pub number: u8,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AspectInfo {
    pub planet1: String,
    pub planet2: String,
    pub aspect: String,
    pub orb: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SynastryAspectInfo {
    pub person1: String,
    pub person2: String,
    pub aspect: String,
    pub orb: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartResponse {
    pub chart_type: String,
    pub date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
    pub ayanamsa: String,
    pub planets: Vec<PlanetInfo>,
    pub houses: Vec<HouseInfo>,
    pub aspects: Vec<AspectInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit: Option<TransitData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_chart: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransitData {
    pub date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub planets: Vec<PlanetInfo>,
    pub aspects: Vec<AspectInfo>,
    pub transit_to_natal_aspects: Vec<AspectInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransitResponse {
    pub chart_type: String,
    pub natal_date: DateTime<Utc>,
    pub transit_date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
    pub ayanamsa: String,
    pub natal_planets: Vec<PlanetInfo>,
    pub transit_planets: Vec<PlanetInfo>,
    pub houses: Vec<HouseInfo>,
    pub natal_aspects: Vec<AspectInfo>,
    pub transit_aspects: Vec<AspectInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_chart: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SynastryResponse {
    pub chart_type: String,
    pub chart1: ChartResponse,
    pub chart2: ChartResponse,
    pub synastries: Vec<SynastryAspectInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg_chart: Option<String>,
}

impl From<PlanetPosition> for PlanetInfo {
    fn from(position: PlanetPosition) -> Self {
        Self {
            name: "Unknown".to_string(), // This will be set by the caller
            longitude: position.longitude,
            latitude: position.latitude,
            speed: position.speed,
            is_retrograde: position.is_retrograde,
            house: position.house,
        }
    }
}
