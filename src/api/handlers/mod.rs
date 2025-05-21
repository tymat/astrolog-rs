use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use log::error;

use crate::api::models::{ChartRequest, ChartResponse, TransitRequest, TransitResponse};
use crate::core::{AstrologError, ChartInfo};

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn generate_chart(
    Json(request): Json<ChartRequest>,
) -> Result<Json<ChartResponse>, AppError> {
    // Convert request to ChartInfo
    let _info = ChartInfo {
        date: DateTime::parse_from_str(&format!("{} {}", request.date, request.time), "%Y-%m-%d %H:%M:%S")
            .map_err(|_| AppError::InvalidInput("Invalid date/time format".into()))?
            .with_timezone(&Utc),
        timezone: request.timezone,
        latitude: request.latitude,
        longitude: request.longitude,
        house_system: request.house_system.parse().map_err(|_| AppError::InvalidInput("Invalid house system".into()))?,
    };

    // TODO: Implement chart generation
    Err(AppError::NotImplemented("Chart generation not yet implemented".into()))
}

pub async fn calculate_transit(
    Json(_request): Json<TransitRequest>,
) -> Result<Json<TransitResponse>, AppError> {
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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
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

        let body = Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

impl From<AstrologError> for AppError {
    fn from(err: AstrologError) -> Self {
        AppError::AstrologError(err)
    }
} 