use axum::{
    routing::post,
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use crate::calc::planets::calculate_planet_positions;
use crate::api::types::{ChartRequest, TransitRequest, SynastryRequest, ChartResponse, PlanetInfo};
use crate::calc::utils::date_to_julian;

async fn generate_natal_chart(
    Json(req): Json<ChartRequest>,
) -> impl IntoResponse {
    let jd = date_to_julian(req.date);
    
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

            let response = ChartResponse {
                chart_type: "natal".to_string(),
                date: req.date,
                latitude: req.latitude,
                longitude: req.longitude,
                house_system: req.house_system.clone(),
                ayanamsa: req.ayanamsa.clone(),
                planets,
                houses: vec![], // TODO: Implement house calculations
                aspects: vec![], // TODO: Implement aspect calculations
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

            let response = ChartResponse {
                chart_type: "transit".to_string(),
                date: req.transit_date,
                latitude: req.latitude,
                longitude: req.longitude,
                house_system: req.house_system.clone(),
                ayanamsa: req.ayanamsa.clone(),
                planets: [natal_planets, transit_planets].concat(),
                houses: vec![], // TODO: Implement house calculations
                aspects: vec![], // TODO: Implement aspect calculations
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

            let response = ChartResponse {
                chart_type: "synastry".to_string(),
                date: req.chart1.date,
                latitude: req.chart1.latitude,
                longitude: req.chart1.longitude,
                house_system: req.chart1.house_system.clone(),
                ayanamsa: req.chart1.ayanamsa.clone(),
                planets: [planets1, planets2].concat(),
                houses: vec![], // TODO: Implement house calculations
                aspects: vec![], // TODO: Implement aspect calculations
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