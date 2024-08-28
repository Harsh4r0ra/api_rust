use actix_web::{web, HttpResponse, Responder};
use crate::models::sos::SOSRequest;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/send_sos", web::post().to(send_sos))
       .route("/get_sos_status/{driver_id}", web::get().to(get_sos_status));
}

async fn send_sos(req: web::Json<SOSRequest>) -> impl Responder {
    HttpResponse::Ok().json(format!("SOS alert from driver {} at location {}", req.driver_id, req.location))
}

async fn get_sos_status(driver_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("SOS status for driver {} is resolved", driver_id))
}
