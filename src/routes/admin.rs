use crate::authentication::Claims;
use crate::db::doctor;
use crate::models::Doctor;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct DoctorQuery {
    search: Option<String>,
    order_column: Option<String>,
    order_dir: Option<String>,
}

#[get("/doctors")]
pub async fn get_doctors(
    data: web::Data<crate::AppState>,
    query: web::Query<DoctorQuery>,
) -> HttpResponse {
    match doctor::get_doctors(
        &data.db,
        query.search.clone(),
        query.order_column.clone(),
        query.order_dir.clone(),
    )
    .await
    {
        Ok(doctors) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": doctors,
            "message": "Danh sách bác sĩ được lấy thành công"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Không thể lấy danh sách bác sĩ: {}", e)
        })),
    }
}

// Lấy thông tin chi tiết một bác sĩ
#[get("/doctors/{id}")]
pub async fn get_doctor_by_id(
    data: web::Data<crate::AppState>,
    id: web::Path<i32>,
) -> HttpResponse {
    match doctor::get_doctor_by_id(&data.db, &id).await {
        Ok(doctor) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": doctor,
            "message": "Thông tin bác sĩ được lấy thành công"
        })),
        Err(e) => HttpResponse::NotFound().json(json!({
            "success": false,
            "message": format!("Không tìm thấy bác sĩ: {}", e)
        })),
    }
}

// Tạo bác sĩ mới
#[post("/doctors")]
pub async fn create_doctor(
    data: web::Data<crate::AppState>,
    doctor: web::Json<Doctor>,
) -> HttpResponse {
    match doctor::create_doctor(&data.db, &doctor).await {
        Ok(_) => HttpResponse::Created().json(json!({
            "success": true,
            "message": "Tạo bác sĩ thành công"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Không thể tạo bác sĩ: {}", e)
        })),
    }
}

// Cập nhật thông tin bác sĩ
#[put("/doctors/{email}")]
pub async fn update_doctor(
    data: web::Data<crate::AppState>,
    email: web::Path<String>,
    doctor: web::Json<Doctor>,
) -> HttpResponse {
    match doctor::update_doctor(&data.db, email.into_inner(), &doctor).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Cập nhật thông tin bác sĩ thành công"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Không thể cập nhật thông tin bác sĩ: {}", e)
        })),
    }
}

// Xóa bác sĩ
#[delete("/doctors/{id}")]
pub async fn delete_doctor(
    data: web::Data<crate::AppState>,
    id: web::Path<i32>,
) -> HttpResponse {
    // Thêm hàm delete_doctor vào module db/doctor.rs
    match doctor::delete_doctor(&data.db, &id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Xóa bác sĩ thành công"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Không thể xóa bác sĩ: {}", e)
        })),
    }
}