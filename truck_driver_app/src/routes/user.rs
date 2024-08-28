use actix_web::{web, HttpResponse, Responder};
use crate::models::user::UserProfile;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/get_user_profile/{user_id}", web::get().to(get_user_profile))
       .route("/update_user_profile", web::post().to(update_user_profile));
}

async fn get_user_profile(user_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(UserProfile {
        user_id: user_id.to_string(),
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    })
}

async fn update_user_profile(req: web::Json<UserProfile>) -> impl Responder {
    HttpResponse::Ok().json(format!("Updated profile for user {}", req.user_id))
}
