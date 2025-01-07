use crate::authentication::Claims;
use crate::db::appointment;
use crate::models::{Appointment, AppointmentCreateForm};
use actix_web::{get, post, web, HttpResponse, Scope};
use chrono::{NaiveDate, Utc};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct AppointmentOrderQuery {
    date: Option<String>,
    speciality_id: Option<i32>,
}

#[post("")]
pub async fn create_appointment(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    body: web::Json<AppointmentCreateForm>,
) -> HttpResponse {
    if claims.role != "staff" && claims.role != "patient" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Staff or patient access required"
        }));
    }

    let appointment_form = body.into_inner();
    
    // Ensure we have a valid date
    if appointment_form.date.is_none() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Date is required"
        }));
    }

    let (numerical_order, appointment_time) = match appointment::calculate_appointment_time(
        &data.db,
        appointment_form.date.unwrap(), // Safe to unwrap after check
        Some(appointment_form.speciality_id),
    )
    .await
    {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Failed to calculate appointment time: {}", e)
            }));
        }
    };

    let appointment = Appointment {
        patient_id: appointment_form.patient_id,
        patient_name: Some(appointment_form.patient_name),
        patient_birthday: Some(appointment_form.patient_birthday),
        patient_phone: Some(appointment_form.patient_phone),
        patient_reason: Some(appointment_form.patient_reason),
        speciality_id: Some(appointment_form.speciality_id),
        numerical_order: Some(numerical_order as i32),
        appointment_time: appointment_time.format("%H:%M").to_string(),
        status: Some("Scheduled".to_string()),
        create_at: Some(Utc::now().naive_utc()),
        update_at: Some(Utc::now().naive_utc()),
        date: appointment_form.date, 
    };

    match appointment::create_appointment(&data.db, appointment).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Appointment created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to create appointment: {}", e)
        })),
    }
}

#[get("/{id}")]
pub async fn get_appointments_of_patient(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    path: web::Path<i32>,
) -> HttpResponse {
    if claims.role != "patient" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Patient access required"
        }));
    }

    let patient_id = path.into_inner();
    match appointment::get_appointments_of_patient(&data.db, patient_id).await {
        Ok(appointments) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Appointments fetched successfully",
            "data": appointments
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to fetch appointments: {}", e)
        })),
    }
}
