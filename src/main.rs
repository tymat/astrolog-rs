mod api;
mod calc;
mod charts;
mod core;
mod data;
mod io;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();
    println!("Starting Astrolog-rs API server");

    // Create router
    let app = api::create_router();

    // Run server
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
