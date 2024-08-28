use actix_web::{web, HttpResponse, Responder};
use crate::models::role::RoleRequest;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/assign_role", web::post().to(assign_role))
       .route("/get_role/{user_id}", web::get().to(get_role));
}

async fn assign_role(req: web::Json<RoleRequest>) -> impl Responder {
    HttpResponse::Ok().json(format!("Assigned role {} to user {}", req.role, req.user_id))
}

async fn get_role(user_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("Role for user {} is admin", user_id))
}
