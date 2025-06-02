use crate::api::types::{
    AspectInfo, ChartRequest, ChartResponse, HouseInfo, PlanetInfo, SynastryRequest,
    SynastryResponse, SynastryAspectInfo, TransitRequest, TransitResponse, TransitData, TransitInfo,
};
use crate::calc::aspects::{calculate_aspects_with_options, calculate_transit_aspects_with_options, calculate_cross_aspects_with_options, calculate_synastry_aspects};
use crate::calc::houses::calculate_houses;
use crate::calc::planets::calculate_planet_positions;
use crate::calc::utils::date_to_julian;
use crate::core::types::HouseSystem;
use crate::utils::logging::log_request_error;
use crate::charts::{generate_natal_svg, generate_synastry_svg, generate_transit_svg};
use actix_web::{
    web, HttpResponse, Responder, middleware,
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    Error
};
use serde_json::json;
use std::cell::RefCell;
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};

thread_local! {
    static CLIENT_IP: RefCell<String> = RefCell::new("unknown".to_string());
}

pub struct IpMiddleware;

impl<S, B> Transform<S, ServiceRequest> for IpMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = IpMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IpMiddlewareService { service }))
    }
}

pub struct IpMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for IpMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();
        
        CLIENT_IP.with(|cell| {
            *cell.borrow_mut() = ip;
        });

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

fn get_client_ip() -> String {
    CLIENT_IP.with(|cell| cell.borrow().clone())
}

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

async fn generate_chart_with_transits(req: web::Json<ChartRequest>) -> impl Responder {
    let jd = date_to_julian(req.date);
    let house_system = parse_house_system(&req.house_system);

    // Calculate natal chart
    match calculate_planet_positions(jd) {
        Ok(natal_positions) => {
            let planets: Vec<PlanetInfo> = natal_positions
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
                Err(e) => {
                    log_request_error(
                        "chart",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &e.to_string(),
                    );
                    return HttpResponse::InternalServerError().body(e.to_string());
                }
            };
            let house_info: Vec<HouseInfo> = houses
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();

            // Calculate natal aspects
            let natal_aspects = calculate_aspects_with_options(&natal_positions, req.include_minor_aspects);
            let aspect_info: Vec<AspectInfo> = natal_aspects
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            // Handle transit data if provided
            let transit_data = if let Some(transit_info) = &req.transit {
                let transit_jd = date_to_julian(transit_info.date);
                
                match calculate_planet_positions(transit_jd) {
                    Ok(transit_positions) => {
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

                        // Calculate transit aspects
                        let transit_aspects = calculate_transit_aspects_with_options(&transit_positions, req.include_minor_aspects);
                        let transit_aspect_info: Vec<AspectInfo> = transit_aspects
                            .iter()
                            .map(|a| AspectInfo {
                                aspect: format!("{:?}", a.aspect_type),
                                orb: a.orb,
                                planet1: a.planet1.clone(),
                                planet2: a.planet2.clone(),
                            })
                            .collect();

                        // Calculate transit-to-natal aspects
                        let cross_aspects = calculate_cross_aspects_with_options(&natal_positions, &transit_positions, req.include_minor_aspects);
                        let cross_aspect_info: Vec<AspectInfo> = cross_aspects
                            .iter()
                            .map(|a| AspectInfo {
                                aspect: format!("{:?}", a.aspect_type),
                                orb: a.orb,
                                planet1: a.planet1.clone(),
                                planet2: a.planet2.clone(),
                            })
                            .collect();

                        Some(TransitData {
                            date: transit_info.date,
                            latitude: transit_info.latitude,
                            longitude: transit_info.longitude,
                            planets: transit_planets,
                            aspects: transit_aspect_info,
                            transit_to_natal_aspects: cross_aspect_info,
                        })
                    }
                    Err(e) => {
                        log_request_error(
                            "chart_transit",
                            &get_client_ip(),
                            &json!(req.0).to_string(),
                            &e.to_string(),
                        );
                        return HttpResponse::InternalServerError().body(format!("Failed to calculate transit positions: {}", e));
                    }
                }
            } else {
                // Use default transit values if no transit data provided
                let default_transit = TransitInfo::default();
                let transit_jd = date_to_julian(default_transit.date);
                
                match calculate_planet_positions(transit_jd) {
                    Ok(transit_positions) => {
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

                        // Calculate transit aspects
                        let transit_aspects = calculate_transit_aspects_with_options(&transit_positions, req.include_minor_aspects);
                        let transit_aspect_info: Vec<AspectInfo> = transit_aspects
                            .iter()
                            .map(|a| AspectInfo {
                                aspect: format!("{:?}", a.aspect_type),
                                orb: a.orb,
                                planet1: a.planet1.clone(),
                                planet2: a.planet2.clone(),
                            })
                            .collect();

                        // Calculate transit-to-natal aspects
                        let cross_aspects = calculate_cross_aspects_with_options(&natal_positions, &transit_positions, req.include_minor_aspects);
                        let cross_aspect_info: Vec<AspectInfo> = cross_aspects
                            .iter()
                            .map(|a| AspectInfo {
                                aspect: format!("{:?}", a.aspect_type),
                                orb: a.orb,
                                planet1: a.planet1.clone(),
                                planet2: a.planet2.clone(),
                            })
                            .collect();

                        Some(TransitData {
                            date: default_transit.date,
                            latitude: default_transit.latitude,
                            longitude: default_transit.longitude,
                            planets: transit_planets,
                            aspects: transit_aspect_info,
                            transit_to_natal_aspects: cross_aspect_info,
                        })
                    }
                    Err(e) => {
                        log_request_error(
                            "chart_default_transit",
                            &get_client_ip(),
                            &json!(req.0).to_string(),
                            &e.to_string(),
                        );
                        return HttpResponse::InternalServerError().body(format!("Failed to calculate default transit positions: {}", e));
                    }
                }
            };

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
                transit: transit_data,
                svg_chart: None, // Will be set below
            };

            // Generate SVG chart
            match generate_natal_svg(&response) {
                Ok(svg_chart) => {
                    let mut final_response = response;
                    final_response.svg_chart = Some(svg_chart);
                    HttpResponse::Ok().json(final_response)
                }
                Err(svg_error) => {
                    log_request_error(
                        "chart",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &format!("SVG generation failed: {}", svg_error),
                    );
                    HttpResponse::InternalServerError().body(format!("SVG generation failed: {}", svg_error))
                }
            }
        }
        Err(e) => {
            log_request_error(
                "chart",
                &get_client_ip(),
                &json!(req.0).to_string(),
                &e.to_string(),
            );
            HttpResponse::InternalServerError().body(e.to_string())
        }
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
                Err(e) => {
                    log_request_error(
                        "natal",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &e.to_string(),
                    );
                    return HttpResponse::InternalServerError().body(e.to_string());
                }
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
            let aspects = calculate_aspects_with_options(&positions, req.include_minor_aspects);
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
                transit: None,
                svg_chart: None, // Will be set below
            };

            // Generate SVG chart
            match generate_natal_svg(&response) {
                Ok(svg_chart) => {
                    let mut final_response = response;
                    final_response.svg_chart = Some(svg_chart);
                    HttpResponse::Ok().json(final_response)
                }
                Err(svg_error) => {
                    log_request_error(
                        "chart",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &format!("SVG generation failed: {}", svg_error),
                    );
                    HttpResponse::InternalServerError().body(format!("SVG generation failed: {}", svg_error))
                }
            }
        }
        Err(e) => {
            log_request_error(
                "natal",
                &get_client_ip(),
                &json!(req.0).to_string(),
                &e.to_string(),
            );
            HttpResponse::InternalServerError().body(e.to_string())
        }
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
                Err(e) => {
                    log_request_error(
                        "transit",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &e.to_string(),
                    );
                    return HttpResponse::InternalServerError().body(e.to_string());
                }
            };
            let house_info: Vec<HouseInfo> = houses
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();

            // Calculate natal aspects
            let natal_aspects = calculate_aspects_with_options(&natal_positions, req.include_minor_aspects);
            let natal_aspect_info: Vec<AspectInfo> = natal_aspects
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            // Calculate transit aspects with tight orbs
            let transit_aspects = calculate_transit_aspects_with_options(&transit_positions, req.include_minor_aspects);
            let transit_aspect_info: Vec<AspectInfo> = transit_aspects
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
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
                houses: house_info,
                natal_aspects: natal_aspect_info,
                transit_aspects: transit_aspect_info,
                svg_chart: None, // Will be set below
            };

            // Generate SVG chart
            match generate_transit_svg(&response) {
                Ok(svg_chart) => {
                    let mut final_response = response;
                    final_response.svg_chart = Some(svg_chart);
                    HttpResponse::Ok().json(final_response)
                }
                Err(svg_error) => {
                    log_request_error(
                        "transit",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &format!("SVG generation failed: {}", svg_error),
                    );
                    HttpResponse::InternalServerError().body(format!("SVG generation failed: {}", svg_error))
                }
            }
        }
        _ => {
            log_request_error(
                "transit",
                &get_client_ip(),
                &json!(req.0).to_string(),
                "Failed to calculate positions",
            );
            HttpResponse::InternalServerError().body("Failed to calculate positions")
        }
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
                Err(e) => {
                    log_request_error(
                        "synastry",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &e.to_string(),
                    );
                    return HttpResponse::InternalServerError().body(e.to_string());
                }
            };
            let houses2 = match calculate_houses(
                jd2,
                req.chart2.latitude,
                req.chart2.longitude,
                house_system,
            ) {
                Ok(h) => h,
                Err(e) => {
                    log_request_error(
                        "synastry",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &e.to_string(),
                    );
                    return HttpResponse::InternalServerError().body(e.to_string());
                }
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

            // Calculate aspects for both charts
            let aspects1 = calculate_aspects_with_options(&positions1, req.chart1.include_minor_aspects);
            let aspects2 = calculate_aspects_with_options(&positions2, req.chart2.include_minor_aspects);
            let aspect_info1: Vec<AspectInfo> = aspects1
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            let aspect_info2: Vec<AspectInfo> = aspects2
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            // Calculate synastry aspects
            let synastry_aspects = calculate_synastry_aspects(&positions1, &positions2, req.chart1.include_minor_aspects);
            let aspect_info: Vec<SynastryAspectInfo> = synastry_aspects
                .iter()
                .map(|a| SynastryAspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    person1: a.planet1.clone(),
                    person2: a.planet2.clone(),
                })
                .collect();

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
                transit: None,
                svg_chart: None, // Will be set below
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
                transit: None,
                svg_chart: None, // Will be set below
            };

            // Generate SVG charts
            let svg_chart1_result = generate_natal_svg(&chart1);
            let svg_chart2_result = generate_natal_svg(&chart2);
            
            match (svg_chart1_result, svg_chart2_result) {
                (Ok(svg_chart1), Ok(svg_chart2)) => {
                    let mut final_chart1 = chart1;
                    final_chart1.svg_chart = Some(svg_chart1);
                    let mut final_chart2 = chart2;
                    final_chart2.svg_chart = Some(svg_chart2);

                    let response = SynastryResponse {
                        chart_type: "synastry".to_string(),
                        chart1: final_chart1,
                        chart2: final_chart2,
                        synastries: aspect_info,
                        svg_chart: None, // Will be set below
                    };

                    // Generate SVG chart for synastry
                    match generate_synastry_svg(&response) {
                        Ok(synastry_svg) => {
                            let mut final_response = response;
                            final_response.svg_chart = Some(synastry_svg);
                            HttpResponse::Ok().json(final_response)
                        }
                        Err(svg_error) => {
                            log_request_error(
                                "synastry",
                                &get_client_ip(),
                                &json!(req.0).to_string(),
                                &format!("Synastry SVG generation failed: {}", svg_error),
                            );
                            HttpResponse::InternalServerError().body(format!("Synastry SVG generation failed: {}", svg_error))
                        }
                    }
                }
                (Err(e), _) | (_, Err(e)) => {
                    log_request_error(
                        "synastry",
                        &get_client_ip(),
                        &json!(req.0).to_string(),
                        &format!("Individual chart SVG generation failed: {}", e),
                    );
                    HttpResponse::InternalServerError().body(format!("Individual chart SVG generation failed: {}", e))
                }
            }
        }
        _ => {
            log_request_error(
                "synastry",
                &get_client_ip(),
                &json!(req.0).to_string(),
                "Failed to calculate positions",
            );
            HttpResponse::InternalServerError().body("Failed to calculate positions")
        }
    }
}

#[allow(dead_code)]
async fn health_check() -> impl Responder {
    // Check Swiss Ephemeris availability
    let ephemeris_status = if std::path::Path::new("./ephe").exists() {
        "available"
    } else {
        "unavailable"
    };
    
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "astrolog-rs",
        "version": env!("CARGO_PKG_VERSION"),
        "checks": {
            "ephemeris": ephemeris_status,
            "server": "running"
        }
    }))
}

#[allow(dead_code)]
pub fn config(cfg: &mut web::ServiceConfig) {
    // Health endpoint at root level for load balancers/monitoring
    cfg.route("/health", web::get().to(health_check));
    
    // API endpoints under /api scope
    cfg.service(
        web::scope("/api")
            .wrap(middleware::Logger::default())
            .wrap(IpMiddleware)
            .route("/chart", web::post().to(generate_chart_with_transits))
            .route("/chart/natal", web::post().to(generate_natal_chart))
            .route("/chart/transit", web::post().to(generate_transit_chart))
            .route("/chart/synastry", web::post().to(generate_synastry_chart)),
    );
}
