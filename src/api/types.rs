use crate::calc::planets::PlanetPosition;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartRequest {
    pub date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
    pub ayanamsa: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransitRequest {
    pub natal_date: DateTime<Utc>,
    pub transit_date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
    pub ayanamsa: String,
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
    pub natal_aspects: Vec<AspectInfo>,
    pub transit_aspects: Vec<AspectInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SynastryResponse {
    pub chart_type: String,
    pub chart1: ChartResponse,
    pub chart2: ChartResponse,
    pub synastries: Vec<AspectInfo>,
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
