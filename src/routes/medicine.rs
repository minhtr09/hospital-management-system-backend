use crate::db::medicine;
use crate::models::{Medicine, MedicineCreateForm};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("/all")]
pub async fn get_medicines(data: web::Data<AppState>) -> HttpResponse {
    match medicine::get_medicines(&data.db).await {
        Ok(medicines) => HttpResponse::Ok().json(medicines),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[get("/{id}")]
pub async fn get_medicine_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    match medicine::get_medicine_by_id(&data.db, path.into_inner()).await {
        Ok(medicine) => HttpResponse::Ok().json(medicine),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[post("")]
pub async fn create_medicine(
    data: web::Data<AppState>,
    body: web::Json<MedicineCreateForm>,
) -> HttpResponse {
    match medicine::create_medicine(&data.db, &body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Medicine created successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_medicine(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<Medicine>,
) -> HttpResponse {
    match medicine::update_medicine(&data.db, path.into_inner(), &body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Medicine updated successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[delete("/{id}")]
pub async fn delete_medicine(data: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    match medicine::delete_medicine(&data.db, path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Medicine deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}