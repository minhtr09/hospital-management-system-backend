use crate::authentication::Claims;
use crate::db::doctor;
use actix_web::{get, web, HttpResponse};
use serde_json::json;

#[get("/self")]
pub async fn get_self_doctor(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    let doctor_id = claims.sub.parse::<i32>().unwrap();

    match doctor::get_doctor_by_id(&data.db, &doctor_id).await {
        Ok(doctor) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": doctor,
            "message": "Doctor retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve doctor: {}", e)
        })),
    }
}
