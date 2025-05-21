use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use crate::api::routes::create_router;

#[tokio::test]
async fn test_health_check() {
    let app = create_router();

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_chart_generation() {
    let app = create_router();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/chart")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "date": "2024-03-15",
            "time": "12:00:00",
            "timezone": 0.0,
            "latitude": 51.5074,
            "longitude": -0.1278,
            "house_system": "placidus"
        }"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_transit_calculation() {
    let app = create_router();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transit")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "birth_date": "1990-01-01",
            "birth_time": "12:00:00",
            "birth_timezone": 0.0,
            "birth_latitude": 51.5074,
            "birth_longitude": -0.1278,
            "transit_date": "2024-03-15",
            "transit_time": "12:00:00",
            "transit_timezone": 0.0
        }"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_invalid_chart_request() {
    let app = create_router();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/chart")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "date": "invalid-date",
            "time": "invalid-time",
            "timezone": "not-a-number",
            "latitude": "not-a-number",
            "longitude": "not-a-number",
            "house_system": "invalid-system"
        }"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_invalid_transit_request() {
    let app = create_router();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transit")
        .header("content-type", "application/json")
        .body(Body::from(r#"{
            "birth_date": "invalid-date",
            "birth_time": "invalid-time",
            "birth_timezone": "not-a-number",
            "birth_latitude": "not-a-number",
            "birth_longitude": "not-a-number",
            "transit_date": "invalid-date",
            "transit_time": "invalid-time",
            "transit_timezone": "not-a-number"
        }"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
} 