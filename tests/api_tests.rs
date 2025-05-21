use actix_web::{test, web, App};
use serde_json::json;
use astrolog_rs::api::server::config;
use astrolog_rs::calc::swiss_ephemeris;

async fn ensure_swiss_ephemeris_initialized() {
    // Ignore error if already initialized
    let _ = swiss_ephemeris::init_swiss_ephemeris();
}

#[actix_web::test]
async fn test_natal_chart_endpoint() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let request = json!({
        "date": "2000-01-01T12:00:00Z",
        "latitude": 40.7128,
        "longitude": -74.0060,
        "house_system": "placidus",
        "ayanamsa": "tropical"
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart/natal")
        .set_json(&request)
        .send_request(&app)
        .await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!("natal_chart_endpoint error: {}", String::from_utf8_lossy(&body));
        panic!("natal_chart_endpoint failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(response.get("planets").is_some());
    assert!(response.get("houses").is_some());
    assert!(response.get("aspects").is_some());
    
    // Check aspects format
    let aspects = response["aspects"].as_array().unwrap();
    for aspect in aspects {
        assert!(aspect.get("planet1").is_some());
        assert!(aspect.get("planet2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());
        
        // Verify planet names are actual planet names
        let planet1 = aspect["planet1"].as_str().unwrap();
        let planet2 = aspect["planet2"].as_str().unwrap();
        assert!(["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"].contains(&planet1));
        assert!(["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"].contains(&planet2));
    }
    
    let planets = response["planets"].as_array().unwrap();
    assert!(!planets.is_empty());
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);
}

#[actix_web::test]
async fn test_transit_chart_endpoint() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let request = json!({
        "natal_date": "2000-01-01T12:00:00Z",
        "transit_date": "2024-01-01T12:00:00Z",
        "latitude": 40.7128,
        "longitude": -74.0060,
        "house_system": "placidus",
        "ayanamsa": "tropical"
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart/transit")
        .set_json(&request)
        .send_request(&app)
        .await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!("transit_chart_endpoint error: {}", String::from_utf8_lossy(&body));
        panic!("transit_chart_endpoint failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(response.get("natal_planets").is_some() || response.get("planets").is_some());
    assert!(response.get("transit_planets").is_some() || response.get("planets").is_some());
    assert!(response.get("aspects").is_some());
}

#[actix_web::test]
async fn test_synastry_chart_endpoint() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let request = json!({
        "chart1": {
            "date": "2000-01-01T12:00:00Z",
            "latitude": 40.7128,
            "longitude": -74.0060,
            "house_system": "placidus",
            "ayanamsa": "tropical"
        },
        "chart2": {
            "date": "1995-01-01T12:00:00Z",
            "latitude": 34.0522,
            "longitude": -118.2437,
            "house_system": "placidus",
            "ayanamsa": "tropical"
        }
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart/synastry")
        .set_json(&request)
        .send_request(&app)
        .await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!("synastry_chart_endpoint error: {}", String::from_utf8_lossy(&body));
        panic!("synastry_chart_endpoint failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(response.get("planets").is_some());
    assert!(response.get("houses").is_some());
    assert!(response.get("aspects").is_some());
    let planets = response["planets"].as_array().unwrap();
    assert!(!planets.is_empty());
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);
}

#[actix_web::test]
async fn test_invalid_input_handling() {
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let invalid_request = json!({
        "date": "invalid-date",
        "latitude": 40.7128,
        "longitude": -74.0060,
        "house_system": "placidus",
        "ayanamsa": "tropical"
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart/natal")
        .set_json(&invalid_request)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());

    let invalid_lat_request = json!({
        "date": "2000-01-01 12:00:00",
        "latitude": 100.0,
        "longitude": -74.0060,
        "house_system": "placidus",
        "ayanamsa": "tropical"
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart/natal")
        .set_json(&invalid_lat_request)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_different_house_systems() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let house_systems = ["placidus", "koch", "equal", "wholesign", "campanus", "regiomontanus"];
    for system in house_systems.iter() {
        let request = json!({
            "date": "2000-01-01T12:00:00Z",
            "latitude": 40.7128,
            "longitude": -74.0060,
            "house_system": system,
            "ayanamsa": "tropical"
        });
        let resp = test::TestRequest::post()
            .uri("/api/chart/natal")
            .set_json(&request)
            .send_request(&app)
            .await;
        if !resp.status().is_success() {
            let body = test::read_body(resp).await;
            println!("house_system {} error: {}", system, String::from_utf8_lossy(&body));
            panic!("house_system {} failed", system);
        }
        let body = test::read_body(resp).await;
        let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let houses = response["houses"].as_array().unwrap();
        assert_eq!(houses.len(), 12);
    }
}

#[actix_web::test]
async fn test_specific_natal_chart() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(
        App::new().configure(config)
    ).await;

    let request = json!({
        "date": "1977-10-24T04:56:00Z",
        "latitude": 14.6486,
        "longitude": 121.0508,
        "house_system": "placidus",
        "ayanamsa": "tropical"
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart/natal")
        .set_json(&request)
        .send_request(&app)
        .await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!("specific_natal_chart error: {}", String::from_utf8_lossy(&body));
        panic!("specific_natal_chart failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    // Print detailed Moon information
    if let Some(planets) = response.get("planets").and_then(|p| p.as_array()) {
        for planet in planets {
            if planet["name"] == "Moon" {
                println!("Detailed Moon position:");
                println!("  Longitude: {}", planet["longitude"]);
                println!("  Speed: {}", planet["speed"]);
                println!("  Is Retrograde: {}", planet["is_retrograde"]);
            }
        }
    }
    
    // Print the full response for verification
    println!("Natal Chart Response: {}", serde_json::to_string_pretty(&response).unwrap());
    
    // Basic validation
    assert!(response.get("planets").is_some());
    assert!(response.get("houses").is_some());
    assert!(response.get("aspects").is_some());
    let planets = response["planets"].as_array().unwrap();
    assert!(!planets.is_empty());
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);
} 