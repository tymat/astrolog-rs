use actix_web::{test, web, App};
use astrolog_rs::api::server::config;
use astrolog_rs::calc::swiss_ephemeris;
use serde_json::json;

async fn ensure_swiss_ephemeris_initialized() {
    // Ignore error if already initialized
    let _ = swiss_ephemeris::init_swiss_ephemeris();
}

#[actix_web::test]
async fn test_natal_chart_endpoint() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

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
        println!(
            "natal_chart_endpoint error: {}",
            String::from_utf8_lossy(&body)
        );
        panic!("natal_chart_endpoint failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check response structure
    assert_eq!(response["chart_type"], "natal");
    assert_eq!(response["date"], "2000-01-01T12:00:00Z");
    assert_eq!(response["latitude"], 40.7128);
    assert_eq!(response["longitude"], -74.0060);
    assert_eq!(response["house_system"], "placidus");
    assert_eq!(response["ayanamsa"], "tropical");

    // Check planets
    let planets = response["planets"].as_array().unwrap();
    assert!(!planets.is_empty());
    for planet in planets {
        assert!(planet.get("name").is_some());
        assert!(planet.get("longitude").is_some());
        assert!(planet.get("latitude").is_some());
        assert!(planet.get("speed").is_some());
        assert!(planet.get("is_retrograde").is_some());
        assert!(planet.get("house").is_some());
    }

    // Check houses
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);
    for house in houses {
        assert!(house.get("number").is_some());
        assert!(house.get("longitude").is_some());
        assert!(house.get("latitude").is_some());
    }

    // Check aspects
    let aspects = response["aspects"].as_array().unwrap();
    for aspect in aspects {
        assert!(aspect.get("planet1").is_some());
        assert!(aspect.get("planet2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());

        // Verify planet names are actual planet names
        let planet1 = aspect["planet1"].as_str().unwrap();
        let planet2 = aspect["planet2"].as_str().unwrap();
        assert!([
            "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
            "Pluto"
        ]
        .contains(&planet1));
        assert!([
            "Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
            "Pluto"
        ]
        .contains(&planet2));
    }

    // Check that SVG chart is generated
    assert!(response.get("svg_chart").is_some());
    let svg_chart = response["svg_chart"].as_str().unwrap();
    assert!(svg_chart.contains("<svg"));
    assert!(svg_chart.contains("</svg>"));
}

#[actix_web::test]
async fn test_transit_chart_endpoint() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

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
        println!(
            "transit_chart_endpoint error: {}",
            String::from_utf8_lossy(&body)
        );
        panic!("transit_chart_endpoint failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check response structure
    assert_eq!(response["chart_type"], "transit");
    assert_eq!(response["natal_date"], "2000-01-01T12:00:00Z");
    assert_eq!(response["transit_date"], "2024-01-01T12:00:00Z");
    assert_eq!(response["latitude"], 40.7128);
    assert_eq!(response["longitude"], -74.0060);
    assert_eq!(response["house_system"], "placidus");
    assert_eq!(response["ayanamsa"], "tropical");

    // Check natal planets
    let natal_planets = response["natal_planets"].as_array().unwrap();
    assert!(!natal_planets.is_empty());
    for planet in natal_planets {
        assert!(planet.get("name").is_some());
        assert!(planet.get("longitude").is_some());
        assert!(planet.get("latitude").is_some());
        assert!(planet.get("speed").is_some());
        assert!(planet.get("is_retrograde").is_some());
        assert!(planet.get("house").is_some());
    }

    // Check transit planets
    let transit_planets = response["transit_planets"].as_array().unwrap();
    assert!(!transit_planets.is_empty());
    for planet in transit_planets {
        assert!(planet.get("name").is_some());
        assert!(planet.get("longitude").is_some());
        assert!(planet.get("latitude").is_some());
        assert!(planet.get("speed").is_some());
        assert!(planet.get("is_retrograde").is_some());
        assert!(planet.get("house").is_some());
    }

    // Check natal aspects
    let natal_aspects = response["natal_aspects"].as_array().unwrap();
    for aspect in natal_aspects {
        assert!(aspect.get("planet1").is_some());
        assert!(aspect.get("planet2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());
    }

    // Check transit aspects
    let transit_aspects = response["transit_aspects"].as_array().unwrap();
    for aspect in transit_aspects {
        assert!(aspect.get("planet1").is_some());
        assert!(aspect.get("planet2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());
    }

    // Check houses
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);
    for house in houses {
        assert!(house.get("number").is_some());
        assert!(house.get("longitude").is_some());
        assert!(house.get("latitude").is_some());
    }

    // Check that SVG chart is generated
    assert!(response.get("svg_chart").is_some());
    let svg_chart = response["svg_chart"].as_str().unwrap();
    assert!(svg_chart.contains("<svg"));
    assert!(svg_chart.contains("</svg>"));
}

#[actix_web::test]
async fn test_synastry_chart_endpoint() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

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
        println!(
            "synastry_chart_endpoint error: {}",
            String::from_utf8_lossy(&body)
        );
        panic!("synastry_chart_endpoint failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check response structure
    assert_eq!(response["chart_type"], "synastry");

    // Check chart1
    let chart1 = &response["chart1"];
    assert_eq!(chart1["chart_type"], "natal");
    assert_eq!(chart1["date"], "2000-01-01T12:00:00Z");
    assert_eq!(chart1["latitude"], 40.7128);
    assert_eq!(chart1["longitude"], -74.0060);
    assert_eq!(chart1["house_system"], "placidus");
    assert_eq!(chart1["ayanamsa"], "tropical");

    // Check chart1 aspects
    let chart1_aspects = chart1["aspects"].as_array().unwrap();
    for aspect in chart1_aspects {
        assert!(aspect.get("planet1").is_some());
        assert!(aspect.get("planet2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());
    }

    // Check chart2
    let chart2 = &response["chart2"];
    assert_eq!(chart2["chart_type"], "natal");
    assert_eq!(chart2["date"], "1995-01-01T12:00:00Z");
    assert_eq!(chart2["latitude"], 34.0522);
    assert_eq!(chart2["longitude"], -118.2437);
    assert_eq!(chart2["house_system"], "placidus");
    assert_eq!(chart2["ayanamsa"], "tropical");

    // Check chart2 aspects
    let chart2_aspects = chart2["aspects"].as_array().unwrap();
    for aspect in chart2_aspects {
        assert!(aspect.get("planet1").is_some());
        assert!(aspect.get("planet2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());
    }

    // Check synastries
    let synastries = response["synastries"].as_array().unwrap();
    for aspect in synastries {
        assert!(aspect.get("person1").is_some());
        assert!(aspect.get("person2").is_some());
        assert!(aspect.get("aspect").is_some());
        assert!(aspect.get("orb").is_some());
    }

    // Check that SVG chart is generated
    assert!(response.get("svg_chart").is_some());
    let svg_chart = response["svg_chart"].as_str().unwrap();
    assert!(svg_chart.contains("<svg"));
    assert!(svg_chart.contains("</svg>"));
}

#[actix_web::test]
async fn test_invalid_input_handling() {
    let app = test::init_service(App::new().configure(config)).await;

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
    let app = test::init_service(App::new().configure(config)).await;

    let house_systems = [
        "placidus",
        "koch",
        "equal",
        "wholesign",
        "campanus",
        "regiomontanus",
    ];
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
            println!(
                "house_system {} error: {}",
                system,
                String::from_utf8_lossy(&body)
            );
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
    let app = test::init_service(App::new().configure(config)).await;

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
        println!(
            "specific_natal_chart error: {}",
            String::from_utf8_lossy(&body)
        );
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
    println!(
        "Natal Chart Response: {}",
        serde_json::to_string_pretty(&response).unwrap()
    );

    // Basic validation
    assert!(response.get("planets").is_some());
    assert!(response.get("houses").is_some());
    assert!(response.get("aspects").is_some());
    let planets = response["planets"].as_array().unwrap();
    assert!(!planets.is_empty());
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);
}

#[actix_web::test]
async fn test_chart_endpoint_with_transits() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

    let request = json!({
        "date": "1977-10-24T04:56:00Z",
        "latitude": 14.6486,
        "longitude": 121.0508,
        "house_system": "placidus",
        "ayanamsa": "tropical",
        "transit": {
            "date": "2025-05-27T12:00:00Z",
            "latitude": 19.49,
            "longitude": -155.99
        }
    });

    let resp = test::TestRequest::post()
        .uri("/api/chart")
        .set_json(&request)
        .send_request(&app)
        .await;

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!(
            "chart_endpoint_with_transits error: {}",
            String::from_utf8_lossy(&body)
        );
        panic!("chart_endpoint_with_transits failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check basic natal chart structure
    assert_eq!(response["chart_type"], "natal");
    assert_eq!(response["date"], "1977-10-24T04:56:00Z");
    assert_eq!(response["latitude"], 14.6486);
    assert_eq!(response["longitude"], 121.0508);
    assert_eq!(response["house_system"], "placidus");
    assert_eq!(response["ayanamsa"], "tropical");

    // Check natal planets
    let planets = response["planets"].as_array().unwrap();
    assert!(!planets.is_empty());
    for planet in planets {
        assert!(planet.get("name").is_some());
        assert!(planet.get("longitude").is_some());
        assert!(planet.get("latitude").is_some());
        assert!(planet.get("speed").is_some());
        assert!(planet.get("is_retrograde").is_some());
    }

    // Check houses
    let houses = response["houses"].as_array().unwrap();
    assert_eq!(houses.len(), 12);

    // Check natal aspects
    let aspects = response["aspects"].as_array().unwrap();
    assert!(!aspects.is_empty());

    // Check transit data
    let transit = response["transit"].as_object().unwrap();
    assert_eq!(transit["date"], "2025-05-27T12:00:00Z");
    assert_eq!(transit["latitude"], 19.49);
    assert_eq!(transit["longitude"], -155.99);

    // Check transit planets
    let transit_planets = transit["planets"].as_array().unwrap();
    assert!(!transit_planets.is_empty());
    assert_eq!(transit_planets.len(), planets.len()); // Should have same number of planets

    // Check transit aspects
    let transit_aspects = transit["aspects"].as_array().unwrap();
    assert!(!transit_aspects.is_empty());

    // Check transit-to-natal aspects
    let cross_aspects = transit["transit_to_natal_aspects"].as_array().unwrap();
    // Cross aspects might be empty if no aspects are within orb, so just check it exists
    // The fact that we can call .as_array().unwrap() means it's a valid array

    // Check that SVG chart is generated
    assert!(response.get("svg_chart").is_some());
    let svg_chart = response["svg_chart"].as_str().unwrap();
    assert!(svg_chart.contains("<svg"));
    assert!(svg_chart.contains("</svg>"));

    println!("Chart with transits response: {}", serde_json::to_string_pretty(&response).unwrap());
}

#[actix_web::test]
async fn test_chart_endpoint_without_transits() {
    ensure_swiss_ephemeris_initialized().await;
    let app = test::init_service(App::new().configure(config)).await;

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

    if !resp.status().is_success() {
        let body = test::read_body(resp).await;
        println!(
            "chart_endpoint_without_transits error: {}",
            String::from_utf8_lossy(&body)
        );
        panic!("chart_endpoint_without_transits failed");
    }
    let body = test::read_body(resp).await;
    let response: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Check basic natal chart structure
    assert_eq!(response["chart_type"], "natal");
    assert_eq!(response["date"], "1977-10-24T04:56:00Z");

    // Check that transit data exists with default values
    let transit = response["transit"].as_object().unwrap();
    assert_eq!(transit["latitude"], 51.45); // Default London coordinates
    assert_eq!(transit["longitude"], 0.05);

    // Check transit planets exist
    let transit_planets = transit["planets"].as_array().unwrap();
    assert!(!transit_planets.is_empty());

    // Check that SVG chart is generated
    assert!(response.get("svg_chart").is_some());
    let svg_chart = response["svg_chart"].as_str().unwrap();
    assert!(svg_chart.contains("<svg"));
    assert!(svg_chart.contains("</svg>"));

    println!("Chart with default transits response: {}", serde_json::to_string_pretty(&response).unwrap());
}
