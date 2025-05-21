use actix_web::{web, HttpResponse, Responder, http::StatusCode};
use chrono::{DateTime, Utc};
use log::error;

use crate::api::models::{ChartRequest, ChartResponse, TransitRequest, TransitResponse};
use crate::core::{AstrologError, ChartInfo};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn generate_chart(
    req: web::Json<ChartRequest>,
) -> Result<HttpResponse, AppError> {
    // Convert request to ChartInfo
    let _info = ChartInfo {
        date: DateTime::parse_from_str(&format!("{} {}", req.date, req.time), "%Y-%m-%d %H:%M:%S")
            .map_err(|_| AppError::InvalidInput("Invalid date/time format".into()))?
            .with_timezone(&Utc),
        timezone: req.timezone,
        latitude: req.latitude,
        longitude: req.longitude,
        house_system: req.house_system.parse().map_err(|_| AppError::InvalidInput("Invalid house system".into()))?,
    };

    // TODO: Implement chart generation
    Err(AppError::NotImplemented("Chart generation not yet implemented".into()))
}

pub async fn calculate_transit(
    req: web::Json<TransitRequest>,
) -> Result<HttpResponse, AppError> {
    // TODO: Implement transit calculation
    Err(AppError::NotImplemented("Transit calculation not yet implemented".into()))
}

// Custom error type for API responses
#[derive(Debug)]
pub enum AppError {
    AstrologError(AstrologError),
    NotImplemented(String),
    InvalidInput(String),
}

impl Responder for AppError {
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        let (status, error_message) = match self {
            AppError::AstrologError(err) => {
                error!("Astrolog error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::NotImplemented(msg) => {
                (StatusCode::NOT_IMPLEMENTED, msg)
            }
            AppError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": error_message
        }))
    }
}

impl From<AstrologError> for AppError {
    fn from(err: AstrologError) -> Self {
        AppError::AstrologError(err)
    }
} 