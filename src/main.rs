use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use routers::{authentication, patient};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use warp::Filter;

mod db;
mod error;
mod middleware;
mod models;
mod routers;
pub struct AppState {
    db: PgPool,
    jwt_secret: String,
}

fn configure_app(cfg: &mut web::ServiceConfig, jwt_secret: String) {
    cfg.service(
        web::scope("/api/information")
            .wrap(AuthMiddleware::new(jwt_secret.clone()))
            .service(patient::get_patients)
            .service(patient::get_patient_by_id)
            .service(patient::update_patient),
    )
    .service(
        web::scope("/api")
            .service(authentication::login)
            .service(authentication::register)
            .service(authentication::reset_password),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // JWT secret
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                jwt_secret: jwt_secret.clone(),
            }))
            .configure(|cfg| configure_app(cfg, jwt_secret.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
