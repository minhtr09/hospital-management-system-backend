    use actix_cors::Cors;
    use actix_web::middleware::Logger;
    use actix_web::{web, App, HttpServer};
    use dotenv::dotenv;
    use middleware::auth::AuthMiddleware;
    use routers::{appointment, authentication, patient, payment, specialty};
    use serde::ser;
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use std::net::SocketAddr;
    use warp::Filter;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::service;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use routes::{appointment, authentication, medicine, patient, payment, service, specialty};
use serde::ser;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use warp::Filter;

    mod db;
    mod error;
    mod middleware;
    mod models;
    mod routes;

    pub struct AppState {
        db: PgPool,
        jwt_secret: String,
    }

    fn configure_app(cfg: &mut web::ServiceConfig, jwt_secret: String) {
        cfg.service(
            web::scope("/api/patient")
                .wrap(AuthMiddleware::new(jwt_secret.clone()))
                .service(patient::get_self_patient)
                .service(patient::get_patients)
                .service(patient::get_patient_by_id)
                .service(patient::update_patient)
                .service(patient::get_patient_by_phone)
                .service(patient::create_patient)
                .service(patient::get_patient_id_by_email),

        )
        .service(
            web::scope("/api/appointment")
                .wrap(AuthMiddleware::new(jwt_secret.clone()))
                .service(appointment::create_appointment)
                .service(appointment::get_appointments_of_patient)
                .service(appointment::get_appointments_by_specialty)
                .service(appointment::update_appointment_status),
        )
        .service(
            web::scope("/api/payment")
                .wrap(AuthMiddleware::new(jwt_secret.clone()))
                .service(payment::get_invoices_of_medical_record)
                .service(payment::create_invoice),
        )
        .service(
            web::scope("/api/specialty")
                .service(specialty::get_specialties)
                .wrap(AuthMiddleware::new(jwt_secret.clone()))
                .service(specialty::get_specialty_by_id)
                .service(specialty::create_speciality)
                .service(specialty::update_speciality)
                .service(specialty::delete_specialty),
        )
        .service(
            web::scope("/api/service")
                .wrap(AuthMiddleware::new(jwt_secret.clone()))
                .service(service::get_services)
                .service(service::get_service_by_id)
                .service(service::create_service)
                .service(service::update_service),
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
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(
                    Cors::default()
                        .allowed_origin("http://localhost:3000")
                        .allow_any_method()
                        .allow_any_header(),
                )
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
