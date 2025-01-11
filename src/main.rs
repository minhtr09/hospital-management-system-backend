use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::service;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use middleware::auth::AuthMiddleware;
use routes::{
    appointment, authentication, doctor, medical_record, medicine, patient, payment, service,
    specialty,admin,
};
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
            .service(appointment::update_appointment_status)
            .service(appointment::update_appointment_treatment_status)
            .service(appointment::get_self_appointments),
    )
    .service(
        web::scope("/api/payment")
            .wrap(AuthMiddleware::new(jwt_secret.clone()))
            .service(payment::get_invoices_of_medical_record)
            .service(payment::create_invoice)
            .service(payment::get_self_invoices),
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
        web::scope("/api/medicine")
            .wrap(AuthMiddleware::new(jwt_secret.clone()))
            .service(medicine::get_medicines)
            .service(medicine::get_medicine_by_id)
            .service(medicine::create_medicine)
            .service(medicine::delete_medicine),
    )
    .service(
        web::scope("/api/medical-record")
            .wrap(AuthMiddleware::new(jwt_secret.clone()))
            .service(medical_record::get_self_medical_records)
            .service(medical_record::update_payment_status)
            .service(medical_record::create_medical_record)
            .service(medical_record::get_medical_record_by_appointment)
            .service(medical_record::update_diagnosis)
            .service(medical_record::get_vital_signs)
            .service(medical_record::create_vital_sign),
    )
    .service(
        web::scope("/api/doctor")
            .wrap(AuthMiddleware::new(jwt_secret.clone()))
            .service(doctor::get_self_doctor)
    )
    .service(
            web::scope("/api/admin")
                .wrap(AuthMiddleware::new(jwt_secret.clone()))
                .service(admin::get_doctors)
                .service(admin::get_doctor_by_id)
                .service(admin::create_doctor)
                .service(admin::update_doctor)
                .service(admin::delete_doctor),
    )
    .service(
        web::scope("/api")
            .service(authentication::login)
            .service(authentication::register)
            .service(authentication::reset_password)
            .service(authentication::get_role)
            .service(
                web::scope("/auth")
                    .wrap(AuthMiddleware::new(jwt_secret.clone()))
                    .service(authentication::update_password)
                    .service(authentication::admin_create_user),
            ),
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
