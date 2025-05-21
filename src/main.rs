mod api;
mod calc;
mod core;
mod utils;
mod io;
mod data;
mod charts;

use tower_http::cors::CorsLayer;
use env_logger::Env;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    println!("Starting Astrolog-rs server on http://127.0.0.1:8080");
    
    let app = api::server::create_router()
        .layer(CorsLayer::permissive());
    
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
