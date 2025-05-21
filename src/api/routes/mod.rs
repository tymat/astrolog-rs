use actix_web::web;
use actix_cors::Cors;

use crate::api::handlers::{calculate_transit, generate_chart, health_check};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(Cors::permissive())
            .route("/health", web::get().to(health_check))
            .route("/v1/chart", web::post().to(generate_chart))
            .route("/v1/transit", web::post().to(calculate_transit))
    );
} 