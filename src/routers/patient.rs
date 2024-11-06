use crate::db::patient;
use crate::models::PatientQuery;
use crate::{authentication::Claims, models::UpdatePatientForm};
use actix_web::{get, put, web, HttpResponse};
use serde_json::json;

#[get("/patients")]
pub async fn get_patients(
    data: web::Data<crate::AppState>,
    query: web::Query<PatientQuery>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Check if user has doctor role
    if claims.role != "doctor" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Doctor access required"
        }));
    }

    match patient::get_patients(
        &data.db,
        query.search.clone(),
        query.order_column.clone(),
        query.order_dir.clone(),
        query.length,
        query.start,
    )
    .await
    {
        Ok(patients) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": patients,
            "message": "Patients retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve patients: {}", e)
        })),
    }
}

#[get("/patients/{id}")]
pub async fn get_patient_by_id(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Check if user has doctor role
    if claims.role != "doctor" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Doctor access required"
        }));
    }

    match patient::get_patient_by_id(&data.db, path.into_inner()).await {
        Ok(patients) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": patients,
            "message": "Patients retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve patients: {}", e)
        })),
    }
}

#[put("/patients/{id}")]
pub async fn update_patient(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    update_req: web::Json<UpdatePatientForm>,
) -> HttpResponse {
    let patient_id = path.into_inner();

    match patient::update_patient(&data.db, update_req.into_inner(), patient_id).await {
        Ok(updated_patient) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": updated_patient,
            "message": "Patient updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to update patient: {}", e)
        })),
    }
}
