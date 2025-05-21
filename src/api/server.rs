use axum::{
    routing::post,
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use crate::calc::planets::calculate_planet_positions;
use crate::calc::houses::calculate_houses;
use crate::calc::aspects::calculate_aspects;
use crate::api::types::{ChartRequest, TransitRequest, SynastryRequest, ChartResponse, PlanetInfo, HouseInfo, AspectInfo};
use crate::calc::utils::date_to_julian;
use crate::core::types::HouseSystem;

fn parse_house_system(system: &str) -> HouseSystem {
    match system.to_lowercase().as_str() {
        "placidus" => HouseSystem::Placidus,
        "koch" => HouseSystem::Koch,
        "equal" => HouseSystem::Equal,
        "wholesign" => HouseSystem::WholeSign,
        "campanus" => HouseSystem::Campanus,
        "regiomontanus" => HouseSystem::Regiomontanus,
        _ => HouseSystem::Placidus, // Default to Placidus
    }
}

async fn generate_natal_chart(
    Json(req): Json<ChartRequest>,
) -> impl IntoResponse {
    let jd = date_to_julian(req.date);
    let house_system = parse_house_system(&req.house_system);
    
    match calculate_planet_positions(jd) {
        Ok(positions) => {
            let planets: Vec<PlanetInfo> = positions.iter()
                .enumerate()
                .map(|(i, pos)| {
                    let mut info: PlanetInfo = (*pos).into();
                    info.name = match i {
                        0 => "Sun".to_string(),
                        1 => "Moon".to_string(),
                        2 => "Mercury".to_string(),
                        3 => "Venus".to_string(),
                        4 => "Mars".to_string(),
                        5 => "Jupiter".to_string(),
                        6 => "Saturn".to_string(),
                        7 => "Uranus".to_string(),
                        8 => "Neptune".to_string(),
                        9 => "Pluto".to_string(),
                        _ => format!("Planet {}", i + 1),
                    };
                    info
                })
                .collect();

            // Calculate houses
            let houses = calculate_houses(jd, req.latitude, req.longitude, house_system);
            let house_info: Vec<HouseInfo> = houses.iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                })
                .collect();

            // Calculate aspects
            let aspects = calculate_aspects(&positions);
            let aspect_info: Vec<AspectInfo> = aspects.iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            let response = ChartResponse {
                chart_type: "natal".to_string(),
                date: req.date,
                latitude: req.latitude,
                longitude: req.longitude,
                house_system: req.house_system.clone(),
                ayanamsa: req.ayanamsa.clone(),
                planets,
                houses: house_info,
                aspects: aspect_info,
            };

            Json(response).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn generate_transit_chart(
    Json(req): Json<TransitRequest>,
) -> impl IntoResponse {
    let natal_jd = date_to_julian(req.natal_date);
    let transit_jd = date_to_julian(req.transit_date);
    let house_system = parse_house_system(&req.house_system);
    
    match (calculate_planet_positions(natal_jd), calculate_planet_positions(transit_jd)) {
        (Ok(natal_positions), Ok(transit_positions)) => {
            let natal_planets: Vec<PlanetInfo> = natal_positions.iter()
                .enumerate()
                .map(|(i, pos)| {
                    let mut info: PlanetInfo = (*pos).into();
                    info.name = format!("Natal {}", match i {
                        0 => "Sun",
                        1 => "Moon",
                        2 => "Mercury",
                        3 => "Venus",
                        4 => "Mars",
                        5 => "Jupiter",
                        6 => "Saturn",
                        7 => "Uranus",
                        8 => "Neptune",
                        9 => "Pluto",
                        _ => "Planet",
                    });
                    info
                })
                .collect();

            let transit_planets: Vec<PlanetInfo> = transit_positions.iter()
                .enumerate()
                .map(|(i, pos)| {
                    let mut info: PlanetInfo = (*pos).into();
                    info.name = format!("Transit {}", match i {
                        0 => "Sun",
                        1 => "Moon",
                        2 => "Mercury",
                        3 => "Venus",
                        4 => "Mars",
                        5 => "Jupiter",
                        6 => "Saturn",
                        7 => "Uranus",
                        8 => "Neptune",
                        9 => "Pluto",
                        _ => "Planet",
                    });
                    info
                })
                .collect();

            // Calculate houses for the transit time
            let houses = calculate_houses(transit_jd, req.latitude, req.longitude, house_system);
            let house_info: Vec<HouseInfo> = houses.iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                })
                .collect();

            // Calculate aspects between natal and transit planets
            let all_positions = [natal_positions, transit_positions].concat();
            let aspects = calculate_aspects(&all_positions);
            let aspect_info: Vec<AspectInfo> = aspects.iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            let response = ChartResponse {
                chart_type: "transit".to_string(),
                date: req.transit_date,
                latitude: req.latitude,
                longitude: req.longitude,
                house_system: req.house_system.clone(),
                ayanamsa: req.ayanamsa.clone(),
                planets: [natal_planets, transit_planets].concat(),
                houses: house_info,
                aspects: aspect_info,
            };

            Json(response).into_response()
        },
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to calculate positions".to_string()).into_response(),
    }
}

async fn generate_synastry_chart(
    Json(req): Json<SynastryRequest>,
) -> impl IntoResponse {
    let jd1 = date_to_julian(req.chart1.date);
    let jd2 = date_to_julian(req.chart2.date);
    let house_system = parse_house_system(&req.chart1.house_system);
    
    match (calculate_planet_positions(jd1), calculate_planet_positions(jd2)) {
        (Ok(positions1), Ok(positions2)) => {
            let planets1: Vec<PlanetInfo> = positions1.iter()
                .enumerate()
                .map(|(i, pos)| {
                    let mut info: PlanetInfo = (*pos).into();
                    info.name = format!("Chart1 {}", match i {
                        0 => "Sun",
                        1 => "Moon",
                        2 => "Mercury",
                        3 => "Venus",
                        4 => "Mars",
                        5 => "Jupiter",
                        6 => "Saturn",
                        7 => "Uranus",
                        8 => "Neptune",
                        9 => "Pluto",
                        _ => "Planet",
                    });
                    info
                })
                .collect();

            let planets2: Vec<PlanetInfo> = positions2.iter()
                .enumerate()
                .map(|(i, pos)| {
                    let mut info: PlanetInfo = (*pos).into();
                    info.name = format!("Chart2 {}", match i {
                        0 => "Sun",
                        1 => "Moon",
                        2 => "Mercury",
                        3 => "Venus",
                        4 => "Mars",
                        5 => "Jupiter",
                        6 => "Saturn",
                        7 => "Uranus",
                        8 => "Neptune",
                        9 => "Pluto",
                        _ => "Planet",
                    });
                    info
                })
                .collect();

            // Calculate houses for the first chart
            let houses = calculate_houses(jd1, req.chart1.latitude, req.chart1.longitude, house_system);
            let house_info: Vec<HouseInfo> = houses.iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                })
                .collect();

            // Calculate aspects between both charts' planets
            let all_positions = [positions1, positions2].concat();
            let aspects = calculate_aspects(&all_positions);
            let aspect_info: Vec<AspectInfo> = aspects.iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            let response = ChartResponse {
                chart_type: "synastry".to_string(),
                date: req.chart1.date,
                latitude: req.chart1.latitude,
                longitude: req.chart1.longitude,
                house_system: req.chart1.house_system.clone(),
                ayanamsa: req.chart1.ayanamsa.clone(),
                planets: [planets1, planets2].concat(),
                houses: house_info,
                aspects: aspect_info,
            };

            Json(response).into_response()
        },
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to calculate positions".to_string()).into_response(),
    }
}

pub fn create_router() -> Router {
    Router::new()
        .route("/api/chart/natal", post(generate_natal_chart))
        .route("/api/chart/transit", post(generate_transit_chart))
        .route("/api/chart/synastry", post(generate_synastry_chart))
} 