use crate::db::patient;
use crate::models::{Patient, PatientForm, PatientQuery};
use crate::{authentication::Claims, models::UpdatePatientForm};
use actix_web::{get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/all")]
pub async fn get_patients(
    data: web::Data<crate::AppState>,
    query: web::Query<PatientQuery>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Check if user has doctor role
    if claims.role != "staff" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Staff access required"
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

#[get("/p{id}")]
pub async fn get_patient_by_id(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Check if user has doctor role
    if claims.role != "staff" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Staff access required"
        }));
    }

    match patient::get_patient_by_id(&data.db, &path.into_inner()).await {
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

#[get("/{email}")]
pub async fn get_patient_id_by_email(
    data: web::Data<crate::AppState>,
    email: web::Path<String>,
) -> HttpResponse {
    match patient::get_patient_id_by_email(&data.db, email.into_inner()).await {
        Ok(patient_id) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": patient_id,
            "message": "Patient ID retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve patient ID: {}", e)
        })),
    }
}

#[put("/{id}")]
pub async fn update_patient(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
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

#[post("")]
pub async fn create_patient(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    body: web::Json<PatientForm>,
) -> HttpResponse {
    // Check if user has staff role
    if claims.role != "staff" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Staff access required"
        }));
    }

    let create_at = chrono::Utc::now().naive_utc();
    let update_at = chrono::Utc::now().naive_utc();

    match patient::create_patient(&data.db, body.into_inner(), create_at, update_at).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Patient created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to create patient: {}", e)
        })),
    }
}

#[get("/{phone}")]
pub async fn get_patient_by_phone(
    data: web::Data<crate::AppState>,
    path: web::Path<String>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Check if user has doctor role
    if claims.role != "staff" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Staff access required"
        }));
    }

    match patient::get_patient_by_phone(&data.db, path.into_inner()).await {
        Ok(patient) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": patient,
            "message": "Patient retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve patient: {}", e)
        })),
    }
}

#[get("/self")]
pub async fn get_self_patient(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    let patient_id = match claims.sub.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "Invalid patient ID"
            }));
        }
    };

    match patient::get_patient_by_id(&data.db, &patient_id).await {
        Ok(patient) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": patient,
            "message": "Patient retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve patient: {}", e)
        })),
    }
}

