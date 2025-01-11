use crate::error::Error;
use crate::models::MedicalRecord;
use crate::{db::medical_record, models::VitalSign};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use super::authentication::Claims;

#[post("")]
pub async fn create_medical_record(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    update_req: web::Json<MedicalRecord>,
) -> impl Responder {
    if claims.role != "doctor" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only doctor can create medical record"
        }));
    }

    match medical_record::create(&data.db, &update_req.into_inner()).await {
        Ok(medical_record_id) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": medical_record_id,
            "message": "Medical record created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to create medical record: {}", e)
        })),
    }
}

#[get("/self")]
pub async fn get_self_medical_records(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    let patient_id = claims.sub.parse::<i32>().unwrap();
    println!("patient_id at get_self_medical_records: {}", patient_id);

    match medical_record::get_by_patient_id(&data.db, patient_id).await {
        Ok(records) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": records,
            "message": "Medical records retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve medical records: {}", e)
        })),
    }
}

#[put("/payment-status/{id}")]
pub async fn update_payment_status(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    if claims.role != "doctor" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only doctor can update payment status"
        }));
    }

    let id = path.into_inner();
    match medical_record::update_payment_status(&data.db, id).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Payment status updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to update payment status: {}", e)
        })),
    }
}

#[get("/vital-signs/{medical_record_id}")]
pub async fn get_vital_signs(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let medical_record_id = path.into_inner();
    match medical_record::get_vital_signs(&data.db, medical_record_id).await {
        Ok(vital_signs) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": vital_signs,
            "message": "Vital signs retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve vital signs: {}", e)
        })),
    }
}

#[post("/vital-signs")]
pub async fn create_vital_sign(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    vital_sign: web::Json<VitalSign>,
) -> impl Responder {
    if claims.role != "doctor" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Only doctor can create vital sign"
        }));
    }

    match medical_record::create_vital_sign(&data.db, &vital_sign.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Vital sign created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to create vital sign: {}", e)
        })),
    }
}
    