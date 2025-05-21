use actix_web::{test, App, web};
use crate::api::routes::config;

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_chart_generation() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/chart")
        .set_json(serde_json::json!({
            "date": "2024-03-15",
            "time": "12:00:00",
            "timezone": 0.0,
            "latitude": 51.5074,
            "longitude": -0.1278,
            "house_system": "placidus"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_IMPLEMENTED);
}

#[actix_web::test]
async fn test_transit_calculation() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/transit")
        .set_json(serde_json::json!({
            "birth_date": "1990-01-01",
            "birth_time": "12:00:00",
            "birth_timezone": 0.0,
            "birth_latitude": 51.5074,
            "birth_longitude": -0.1278,
            "transit_date": "2024-03-15",
            "transit_time": "12:00:00",
            "transit_timezone": 0.0
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NOT_IMPLEMENTED);
}

#[actix_web::test]
async fn test_invalid_chart_request() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/chart")
        .set_json(serde_json::json!({
            "date": "invalid-date",
            "time": "invalid-time",
            "timezone": "not-a-number",
            "latitude": "not-a-number",
            "longitude": "not-a-number",
            "house_system": "invalid-system"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_invalid_transit_request() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/transit")
        .set_json(serde_json::json!({
            "birth_date": "invalid-date",
            "birth_time": "invalid-time",
            "birth_timezone": "not-a-number",
            "birth_latitude": "not-a-number",
            "birth_longitude": "not-a-number",
            "transit_date": "invalid-date",
            "transit_time": "invalid-time",
            "transit_timezone": "not-a-number"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::BAD_REQUEST);
} 