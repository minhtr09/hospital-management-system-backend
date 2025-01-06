use actix_web::{get, post, put, delete, web, HttpResponse};
use crate::AppState;
use crate::models::{Service, ServiceCreateForm};
use crate::db::service;
use serde_json::json;

#[get("/all")]
pub async fn get_services(data: web::Data<AppState>) -> HttpResponse {
    match service::get_services(&data.db).await {
        Ok(services) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": services,
            "message": "Services retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve services: {}", e)
        })),
    }
}

#[get("/{id}")]
pub async fn get_service_by_id(
    data: web::Data<AppState>,
    path: web::Path<i32>
) -> HttpResponse {
    match service::get_service_by_id(&data.db, path.into_inner()).await {
        Ok(service) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": service,
            "message": "Service retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve service: {}", e)
        })),
    }
}

#[post("")]
pub async fn create_service(
    data: web::Data<AppState>,
    body: web::Json<ServiceCreateForm>
) -> HttpResponse {
    match service::create_service(&data.db, &body).await {
        Ok(id) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": id,
            "message": "Service created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to create service: {}", e)
        })),
    }
}

#[put("/{id}")]
pub async fn update_service(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<ServiceCreateForm>
) -> HttpResponse {
    match service::update_service(&data.db, path.into_inner(), &body).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Service updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to update service: {}", e)
        })),
    }
}

#[delete("/{id}")]
pub async fn delete_service(
    data: web::Data<AppState>,
    path: web::Path<i32>
) -> HttpResponse {
    match service::delete_service(&data.db, path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Service deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to delete service: {}", e)
        })),
    }
}