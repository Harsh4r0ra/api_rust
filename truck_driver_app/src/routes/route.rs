use actix_web::{web, HttpResponse, Responder};
use crate::models::route::Route;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/get_route/{driver_id}", web::get().to(get_route))
       .route("/update_route", web::post().to(update_route));
}

async fn get_route(driver_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(Route {
        driver_id: driver_id.to_string(),
        route_details: "Route details for driver".to_string(),
    })
}

async fn update_route(req: web::Json<Route>) -> impl Responder {
    HttpResponse::Ok().json(format!("Updated route for driver {}", req.driver_id))
}
