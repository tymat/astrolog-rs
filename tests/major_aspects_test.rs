use actix_web::{test, web, App};
use astrolog_rs::api::server::config;
use astrolog_rs::calc::swiss_ephemeris;
use serde_json::json;

async fn ensure_swiss_ephemeris_initialized() {
    // Ignore error if already initialized
    let _ = swiss_ephemeris::init_swiss_ephemeris();
}

#[actix_web::test]
async fn test_major_aspects_only() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

    let request = json!({
        "date": "1977-10-24T04:56:00Z",
        "latitude": 14.6486,
        "longitude": 121.0508,
        "house_system": "placidus",
        "ayanamsa": "tropical",
        "include_minor_aspects": false
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart")
        .set_json(&request)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check that we have aspects
    let aspects = response["aspects"].as_array().unwrap();
    assert!(!aspects.is_empty());

    // Check that all aspects are major aspects only
    for aspect in aspects {
        let aspect_type = aspect["aspect"].as_str().unwrap();
        assert!(
            aspect_type == "Conjunction" ||
            aspect_type == "Sextile" ||
            aspect_type == "Square" ||
            aspect_type == "Trine" ||
            aspect_type == "Opposition",
            "Found non-major aspect: {}", aspect_type
        );
    }

    // Check transit aspects too
    let transit = response["transit"].as_object().unwrap();
    let transit_aspects = transit["aspects"].as_array().unwrap();
    for aspect in transit_aspects {
        let aspect_type = aspect["aspect"].as_str().unwrap();
        assert!(
            aspect_type == "Conjunction" ||
            aspect_type == "Sextile" ||
            aspect_type == "Square" ||
            aspect_type == "Trine" ||
            aspect_type == "Opposition",
            "Found non-major transit aspect: {}", aspect_type
        );
    }

    // Check cross aspects
    let cross_aspects = transit["transit_to_natal_aspects"].as_array().unwrap();
    for aspect in cross_aspects {
        let aspect_type = aspect["aspect"].as_str().unwrap();
        assert!(
            aspect_type == "Conjunction" ||
            aspect_type == "Sextile" ||
            aspect_type == "Square" ||
            aspect_type == "Trine" ||
            aspect_type == "Opposition",
            "Found non-major cross aspect: {}", aspect_type
        );
    }

    println!("Major aspects only test passed. Found {} natal aspects, {} transit aspects, {} cross aspects", 
             aspects.len(), transit_aspects.len(), cross_aspects.len());
}

#[actix_web::test]
async fn test_with_minor_aspects() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

    let request = json!({
        "date": "1977-10-24T04:56:00Z",
        "latitude": 14.6486,
        "longitude": 121.0508,
        "house_system": "placidus",
        "ayanamsa": "tropical",
        "include_minor_aspects": true
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart")
        .set_json(&request)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check that we have aspects
    let aspects = response["aspects"].as_array().unwrap();
    assert!(!aspects.is_empty());

    // Check that we have both major and minor aspects
    let mut has_major = false;
    let mut has_minor = false;

    for aspect in aspects {
        let aspect_type = aspect["aspect"].as_str().unwrap();
        if aspect_type == "Conjunction" || aspect_type == "Sextile" || 
           aspect_type == "Square" || aspect_type == "Trine" || aspect_type == "Opposition" {
            has_major = true;
        } else {
            has_minor = true;
        }
    }

    assert!(has_major, "Should have major aspects");
    assert!(has_minor, "Should have minor aspects when include_minor_aspects is true");

    println!("Minor aspects test passed. Found {} total aspects (major + minor)", aspects.len());
}

#[actix_web::test]
async fn test_default_behavior() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

    // Test without include_minor_aspects field (should default to false)
    let request = json!({
        "date": "1977-10-24T04:56:00Z",
        "latitude": 14.6486,
        "longitude": 121.0508,
        "house_system": "placidus",
        "ayanamsa": "tropical"
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart")
        .set_json(&request)
        .send_request(&app)
        .await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check that we have aspects
    let aspects = response["aspects"].as_array().unwrap();
    assert!(!aspects.is_empty());

    // Check that all aspects are major aspects only (default behavior)
    for aspect in aspects {
        let aspect_type = aspect["aspect"].as_str().unwrap();
        assert!(
            aspect_type == "Conjunction" ||
            aspect_type == "Sextile" ||
            aspect_type == "Square" ||
            aspect_type == "Trine" ||
            aspect_type == "Opposition",
            "Found non-major aspect in default behavior: {}", aspect_type
        );
    }

    println!("Default behavior test passed. Only major aspects found as expected.");
} 