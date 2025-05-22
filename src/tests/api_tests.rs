use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;
use crate::api::server::config;
use crate::api::types::{ChartRequest, TransitRequest, SynastryRequest};

#[actix_web::test]
async fn test_natal_chart_invalid_date() {
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    // Test with invalid date
    let req = test::TestRequest::post()
        .uri("/api/chart/natal")
        .set_json(json!({
            "date": "invalid-date",
            "latitude": 0.0,
            "longitude": 0.0,
            "house_system": "placidus",
            "ayanamsa": "tropical"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_transit_chart_invalid_coordinates() {
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    // Test with invalid coordinates
    let req = test::TestRequest::post()
        .uri("/api/chart/transit")
        .set_json(json!({
            "natal_date": "2024-01-01T00:00:00Z",
            "transit_date": "2024-01-02T00:00:00Z",
            "latitude": 1000.0,  // Invalid latitude
            "longitude": 0.0,
            "house_system": "placidus",
            "ayanamsa": "tropical"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn test_synastry_chart_missing_data() {
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    // Test with missing required data
    let req = test::TestRequest::post()
        .uri("/api/chart/synastry")
        .set_json(json!({
            "chart1": {
                "date": "2024-01-01T00:00:00Z",
                "latitude": 0.0,
                "longitude": 0.0,
                "house_system": "placidus",
                "ayanamsa": "tropical"
            }
            // Missing chart2 data
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_natal_chart_success() {
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/chart/natal")
        .set_json(json!({
            "date": "2024-01-01T00:00:00Z",
            "latitude": 0.0,
            "longitude": 0.0,
            "house_system": "placidus",
            "ayanamsa": "tropical"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let chart_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    // Verify response structure
    assert!(chart_response.get("chart_type").is_some());
    assert!(chart_response.get("planets").is_some());
    assert!(chart_response.get("houses").is_some());
    assert!(chart_response.get("aspects").is_some());
}

#[actix_web::test]
async fn test_error_logging() {
    // Ensure log file exists and is empty
    let log_path = "request_errors.log";
    std::fs::write(log_path, "").expect("Failed to create log file");

    let app = test::init_service(
        App::new().configure(config)
    ).await;

    // Make a request that will fail
    let req = test::TestRequest::post()
        .uri("/api/chart/transit")
        .set_json(json!({
            "natal_date": "2024-01-01T00:00:00Z",
            "transit_date": "2024-01-02T00:00:00Z",
            "latitude": 1000.0,  // Invalid latitude
            "longitude": 0.0,
            "house_system": "placidus",
            "ayanamsa": "tropical"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

    // Wait longer for the log to be written and flush
    std::thread::sleep(std::time::Duration::from_millis(1000));

    // Verify that the error was logged
    let log_contents = std::fs::read_to_string(log_path)
        .expect("Failed to read log file");
    
    println!("Log contents: {}", log_contents); // Debug output
    
    // The log should contain the endpoint and the error details
    assert!(log_contents.contains("Endpoint: transit"), "Log should contain endpoint name");
    assert!(log_contents.contains("IP: unknown"), "Log should contain IP address");
    assert!(log_contents.contains("Error:"), "Log should contain error message");
    assert!(log_contents.contains("Invalid latitude"), "Log should contain error about invalid latitude");
} 