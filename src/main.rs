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
use std::env;
use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize Swiss Ephemeris
    if let Err(e) = swiss_ephemeris::init_swiss_ephemeris() {
        eprintln!("Failed to initialize Swiss Ephemeris: {}", e);
        std::process::exit(1);
    }

    // Get number of workers from environment or use number of CPU cores * 2
    let workers = env::var("WORKERS")
        .ok()
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or_else(|| num_cpus::get() * 2);

    // Create a semaphore to limit concurrent calculations
    let max_concurrent = env::var("MAX_CONCURRENT")
        .ok()
        .and_then(|m| m.parse::<usize>().ok())
        .unwrap_or(1000);
    let semaphore = Arc::new(Semaphore::new(max_concurrent));

    println!("Starting Astrolog-rs server on http://127.0.0.1:4008 with {} workers", workers);
    println!("Maximum concurrent calculations: {}", max_concurrent);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .app_data(Data::new(semaphore.clone()))
            .configure(config)
    })
    .workers(workers)
    .keep_alive(std::time::Duration::from_secs(120))
    .client_request_timeout(std::time::Duration::from_secs(120))
    .client_shutdown(5000)
    .backlog(8192)
    .bind("127.0.0.1:4008")?
    .run()
    .await
}
