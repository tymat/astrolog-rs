mod api;
mod calc;
mod charts;
mod core;
mod data;
mod io;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};
use astrolog_rs::api::server::config;
use astrolog_rs::calc::swiss_ephemeris;
use crate::api::queue::{QueueConfig, RequestQueue};
use env_logger::Env;
use std::env;
use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::Semaphore;
use actix_web::middleware::Logger;
use actix_web::middleware::Compress;
use actix_web::middleware::NormalizePath;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Initialize Swiss Ephemeris
    if let Err(e) = swiss_ephemeris::init_swiss_ephemeris() {
        eprintln!("Failed to initialize Swiss Ephemeris: {}", e);
        std::process::exit(1);
    }

    // Get number of workers from environment or use number of CPU cores
    let workers = env::var("WORKERS")
        .ok()
        .and_then(|w| w.parse::<usize>().ok())
        .unwrap_or_else(|| num_cpus::get());

    // Create request queue configuration
    let queue_config = QueueConfig {
        max_queue_size: env::var("MAX_QUEUE_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(10000),
        max_wait_time: std::time::Duration::from_secs(
            env::var("MAX_WAIT_TIME")
                .ok()
                .and_then(|t| t.parse::<u64>().ok())
                .unwrap_or(30)
        ),
        priority_levels: 3,
    };

    // Create a semaphore to limit concurrent calculations
    let max_concurrent = env::var("MAX_CONCURRENT")
        .ok()
        .and_then(|m| m.parse::<usize>().ok())
        .unwrap_or(500);
    let semaphore = Arc::new(Semaphore::new(max_concurrent));

    // Create request queue
    let request_queue = Arc::new(RequestQueue::new(queue_config, max_concurrent));

    println!("Starting Astrolog-rs server on http://127.0.0.1:4008 with {} workers", workers);
    println!("Maximum concurrent calculations: {}", max_concurrent);
    println!("Maximum queue size: {}", request_queue.max_queue_size());
    println!("Maximum wait time: {} seconds", request_queue.max_wait_time().as_secs());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::trim())
            .app_data(Data::new(semaphore.clone()))
            .app_data(Data::new(request_queue.clone()))
            .configure(config)
    })
    .workers(workers)
    .keep_alive(std::time::Duration::from_secs(75))
    .client_request_timeout(std::time::Duration::from_secs(60))
    .client_shutdown(5000)
    .backlog(16384)
    .bind("127.0.0.1:4008")?
    .run()
    .await
}
