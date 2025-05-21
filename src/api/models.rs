use serde::{Deserialize, Serialize};
use crate::core::Chart;

#[derive(Debug, Deserialize)]
pub struct ChartRequest {
    pub date: String,
    pub time: String,
    pub timezone: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub house_system: String,
}

#[derive(Debug, Serialize)]
pub struct ChartResponse {
    pub chart: Chart,
}

#[derive(Debug, Deserialize)]
pub struct TransitRequest {
    pub birth_date: String,
    pub birth_time: String,
    pub birth_timezone: f64,
    pub birth_latitude: f64,
    pub birth_longitude: f64,
    pub transit_date: String,
    pub transit_time: String,
    pub transit_timezone: f64,
}

#[derive(Debug, Serialize)]
pub struct TransitResponse {
    pub birth_chart: Chart,
    pub transit_chart: Chart,
    pub aspects: Vec<(String, String, String, f64, bool)>, // (planet1, planet2, aspect_type, orb, applying)
} 