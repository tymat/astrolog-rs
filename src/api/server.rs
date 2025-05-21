use crate::api::types::{
    AspectInfo, ChartRequest, ChartResponse, HouseInfo, PlanetInfo, SynastryRequest,
    SynastryResponse, TransitRequest, TransitResponse,
};
use crate::calc::aspects::calculate_aspects;
use crate::calc::aspects::AspectType;
use crate::calc::houses::calculate_houses;
use crate::calc::planets::calculate_planet_positions;
use crate::calc::utils::date_to_julian;
use crate::core::types::HouseSystem;
use actix_web::{web, HttpResponse, Responder};

#[allow(dead_code)]
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

#[allow(dead_code)]
async fn generate_natal_chart(req: web::Json<ChartRequest>) -> impl Responder {
    let jd = date_to_julian(req.date);
    let house_system = parse_house_system(&req.house_system);

    match calculate_planet_positions(jd) {
        Ok(positions) => {
            let planets: Vec<PlanetInfo> = positions
                .iter()
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
            let houses = match calculate_houses(jd, req.latitude, req.longitude, house_system) {
                Ok(h) => h,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            };
            let _house_info: Vec<HouseInfo> = houses
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();

            // Calculate aspects
            let aspects = calculate_aspects(&positions);
            let aspect_info: Vec<AspectInfo> = aspects
                .iter()
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
                houses: _house_info,
                aspects: aspect_info,
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[allow(dead_code)]
async fn generate_transit_chart(req: web::Json<TransitRequest>) -> impl Responder {
    let natal_jd = date_to_julian(req.natal_date);
    let transit_jd = date_to_julian(req.transit_date);
    let house_system = parse_house_system(&req.house_system);

    match (
        calculate_planet_positions(natal_jd),
        calculate_planet_positions(transit_jd),
    ) {
        (Ok(natal_positions), Ok(transit_positions)) => {
            let natal_planets: Vec<PlanetInfo> = natal_positions
                .iter()
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

            let transit_planets: Vec<PlanetInfo> = transit_positions
                .iter()
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

            // Calculate houses for the natal chart
            let houses = match calculate_houses(natal_jd, req.latitude, req.longitude, house_system)
            {
                Ok(h) => h,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            };
            let _house_info: Vec<HouseInfo> = houses
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();

            // Calculate aspects between natal and transit planets
            let all_positions = [natal_positions.clone(), transit_positions].concat();
            let aspects = calculate_aspects(&all_positions);
            let aspect_info: Vec<AspectInfo> = aspects
                .iter()
                .map(|a| {
                    // Map indices to correct planet names based on which chart they come from
                    let planet1 = if a.planet1.starts_with("Planet") {
                        let idx = a.planet1[6..].parse::<usize>().unwrap() - 1;
                        if idx < natal_positions.len() {
                            match idx {
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
                                _ => format!("Planet{}", idx + 1),
                            }
                        } else {
                            let idx = idx - natal_positions.len();
                            match idx {
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
                                _ => format!("Planet{}", idx + 1),
                            }
                        }
                    } else {
                        a.planet1.clone()
                    };

                    let planet2 = if a.planet2.starts_with("Planet") {
                        let idx = a.planet2[6..].parse::<usize>().unwrap() - 1;
                        if idx < natal_positions.len() {
                            match idx {
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
                                _ => format!("Planet{}", idx + 1),
                            }
                        } else {
                            let idx = idx - natal_positions.len();
                            match idx {
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
                                _ => format!("Planet{}", idx + 1),
                            }
                        }
                    } else {
                        a.planet2.clone()
                    };

                    AspectInfo {
                        aspect: format!("{:?}", a.aspect_type),
                        orb: a.orb,
                        planet1,
                        planet2,
                    }
                })
                .collect();

            let response = TransitResponse {
                chart_type: "transit".to_string(),
                natal_date: req.natal_date,
                transit_date: req.transit_date,
                latitude: req.latitude,
                longitude: req.longitude,
                house_system: req.house_system.clone(),
                ayanamsa: req.ayanamsa.clone(),
                natal_planets,
                transit_planets,
                aspects: aspect_info,
            };

            HttpResponse::Ok().json(response)
        }
        _ => HttpResponse::InternalServerError().body("Failed to calculate positions"),
    }
}

#[allow(dead_code)]
async fn generate_synastry_chart(req: web::Json<SynastryRequest>) -> impl Responder {
    let jd1 = date_to_julian(req.chart1.date);
    let jd2 = date_to_julian(req.chart2.date);
    let house_system = parse_house_system(&req.chart1.house_system);

    match (
        calculate_planet_positions(jd1),
        calculate_planet_positions(jd2),
    ) {
        (Ok(positions1), Ok(positions2)) => {
            let planets1: Vec<PlanetInfo> = positions1
                .iter()
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

            let planets2: Vec<PlanetInfo> = positions2
                .iter()
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

            // Calculate houses for both charts
            let houses1 = match calculate_houses(
                jd1,
                req.chart1.latitude,
                req.chart1.longitude,
                house_system,
            ) {
                Ok(h) => h,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            };
            let houses2 = match calculate_houses(
                jd2,
                req.chart2.latitude,
                req.chart2.longitude,
                house_system,
            ) {
                Ok(h) => h,
                Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
            };

            let _house_info1: Vec<HouseInfo> = houses1
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();
            let _house_info2: Vec<HouseInfo> = houses2
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();

            // Calculate aspects for chart1
            let aspects1 = calculate_aspects(&positions1);
            let aspect_info1: Vec<AspectInfo> = aspects1
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            // Calculate aspects for chart2
            let aspects2 = calculate_aspects(&positions2);
            let aspect_info2: Vec<AspectInfo> = aspects2
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            // Calculate aspects between both charts' planets
            let mut aspect_info = Vec::new();

            // Calculate aspects between each planet in chart1 and each planet in chart2
            for (i, pos1) in positions1.iter().enumerate() {
                for (j, pos2) in positions2.iter().enumerate() {
                    let diff = (pos1.longitude - pos2.longitude).abs() % 360.0;
                    let min_diff = diff.min(360.0 - diff);

                    // Check each aspect type
                    for aspect_type in [
                        AspectType::Conjunction,
                        AspectType::SemiSextile,
                        AspectType::SemiSquare,
                        AspectType::Sextile,
                        AspectType::Quintile,
                        AspectType::Square,
                        AspectType::BiQuintile,
                        AspectType::Trine,
                        AspectType::Sesquisquare,
                        AspectType::Quincunx,
                        AspectType::Opposition,
                        AspectType::Septile,
                        AspectType::BiSeptile,
                        AspectType::TriSeptile,
                        AspectType::Novile,
                        AspectType::BiNovile,
                        AspectType::QuadNovile,
                    ]
                    .iter()
                    {
                        let aspect_angle = aspect_type.angle();
                        let orb = aspect_type.orb();
                        if (min_diff - aspect_angle).abs() <= orb {
                            aspect_info.push(AspectInfo {
                                aspect: format!("{:?}", aspect_type),
                                orb: (min_diff - aspect_angle).abs(),
                                planet1: planets1[i].name.clone(),
                                planet2: planets2[j].name.clone(),
                            });
                        }
                    }
                }
            }

            // Print debug information
            println!("Synastry aspects found: {}", aspect_info.len());

            let chart1 = ChartResponse {
                chart_type: "natal".to_string(),
                date: req.chart1.date,
                latitude: req.chart1.latitude,
                longitude: req.chart1.longitude,
                house_system: req.chart1.house_system.clone(),
                ayanamsa: req.chart1.ayanamsa.clone(),
                planets: planets1,
                houses: _house_info1,
                aspects: aspect_info1,
            };

            let chart2 = ChartResponse {
                chart_type: "natal".to_string(),
                date: req.chart2.date,
                latitude: req.chart2.latitude,
                longitude: req.chart2.longitude,
                house_system: req.chart2.house_system.clone(),
                ayanamsa: req.chart2.ayanamsa.clone(),
                planets: planets2,
                houses: _house_info2,
                aspects: aspect_info2,
            };

            let response = SynastryResponse {
                chart_type: "synastry".to_string(),
                chart1,
                chart2,
                synastries: aspect_info,
            };

            HttpResponse::Ok().json(response)
        }
        _ => HttpResponse::InternalServerError().body("Failed to calculate positions"),
    }
}

#[allow(dead_code)]
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/chart/natal", web::post().to(generate_natal_chart))
            .route("/chart/transit", web::post().to(generate_transit_chart))
            .route("/chart/synastry", web::post().to(generate_synastry_chart)),
    );
}
