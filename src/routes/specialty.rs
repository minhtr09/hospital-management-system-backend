use crate::db::specialty;
use crate::models::Speciality;
use crate::AppState;
use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::error::Error;

#[get("/all")]
pub async fn get_specialties(
    data: web::Data<AppState>
) -> HttpResponse {
    match specialty::get_specialties(&data.db).await {
        Ok(specialties) => HttpResponse::Ok().json(specialties),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_specialty_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    match specialty::get_specialty(&data.db, path.into_inner()).await {
        Ok(specialty) => HttpResponse::Ok().json(specialty),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("")]
pub async fn create_speciality(
    data: web::Data<AppState>,
    body: web::Json<Speciality>,
) -> HttpResponse {
    match specialty::create_specialty(&data.db, &body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Specialty created successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("/update")]
pub async fn update_speciality(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<Speciality>,
) -> HttpResponse {
    match specialty::update_specialty(&data.db, path.into_inner(), &body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Specialty updated successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/delete")]
pub async fn delete_specialty(data: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    match specialty::delete_specialty(&data.db, path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Specialty deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}