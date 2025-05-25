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
use crate::utils::logging::log_request_error;
use actix_web::{
    web, HttpResponse, Responder, http::StatusCode, middleware,
    dev::{ServiceRequest, ServiceResponse, Service, Transform},
    Error, HttpServer
};
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;
use std::future::{ready, Ready, Future};
use std::pin::Pin;
use std::task::{Context, Poll};
use chrono;

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
            let _house_info: Vec<HouseInfo> = houses
                .iter()
                .map(|h| HouseInfo {
                    number: h.number,
                    longitude: h.longitude,
                    latitude: h.latitude,
                })
                .collect();

            // Calculate natal aspects
            let natal_aspects = calculate_aspects(&natal_positions);
            let natal_aspect_info: Vec<AspectInfo> = natal_aspects
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
                })
                .collect();

            // Calculate transit aspects
            let transit_aspects = calculate_aspects(&transit_positions);
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
                natal_aspects: natal_aspect_info,
                transit_aspects: transit_aspect_info,
            };

            HttpResponse::Ok().json(response)
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
            let aspects1 = calculate_aspects(&positions1);
            let aspects2 = calculate_aspects(&positions2);
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
            let synastry_aspects = calculate_aspects(&positions1);
            let aspect_info: Vec<AspectInfo> = synastry_aspects
                .iter()
                .map(|a| AspectInfo {
                    aspect: format!("{:?}", a.aspect_type),
                    orb: a.orb,
                    planet1: a.planet1.clone(),
                    planet2: a.planet2.clone(),
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
            .route("/chart/natal", web::post().to(generate_natal_chart))
            .route("/chart/transit", web::post().to(generate_transit_chart))
            .route("/chart/synastry", web::post().to(generate_synastry_chart)),
    );
}
