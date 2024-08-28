use actix_web::{App, HttpServer};

mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::roles::init)
            .configure(routes::user::init)
            .configure(routes::route::init)
            .configure(routes::sos::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
