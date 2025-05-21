mod api;
mod calc;
mod charts;
mod core;
mod data;
mod io;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use astrolog_rs::api::server::config;
use astrolog_rs::calc::swiss_ephemeris;
use env_logger::Env;
use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize Swiss Ephemeris
    if let Err(e) = swiss_ephemeris::init_swiss_ephemeris() {
        eprintln!("Failed to initialize Swiss Ephemeris: {}", e);
        std::process::exit(1);
    }

    println!("Starting Astrolog-rs server on http://127.0.0.1:4008");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .configure(config)
    })
    .bind("127.0.0.1:4008")?
    .run()
    .await
}
