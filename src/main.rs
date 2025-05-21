mod api;
mod calc;
mod core;
mod utils;
mod io;
mod data;
mod charts;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use env_logger::Env;
use astrolog_rs::api::server::config;
use astrolog_rs::calc::swiss_ephemeris;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // Initialize Swiss Ephemeris
    if let Err(e) = swiss_ephemeris::init_swiss_ephemeris() {
        eprintln!("Failed to initialize Swiss Ephemeris: {}", e);
        std::process::exit(1);
    }

    println!("Starting Astrolog-rs server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
