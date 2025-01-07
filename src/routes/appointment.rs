use crate::authentication::Claims;
use crate::db::{appointment, patient};
use crate::models::{Appointment, AppointmentCreateForm, AppointmentResponse, Patient};
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;
use serde_json::json;

// #[get("/patient/{id}")]
// pub async fn get_appointments_of_patient(
//     data: web::Data<crate::AppState>,
//     path: web::Path<i32>,
//     claims: web::ReqData<Claims>,
// ) -> HttpResponse {

// }

#[post("")]
pub async fn create_appointment(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    body: web::Json<AppointmentCreateForm>,
) -> HttpResponse {
    if claims.role != "staff" && claims.role != "patient" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Staff access required"
        }));
    }

    let appointment_form = body.into_inner();
    let (numerical_order, appointment_time) = match appointment::calculate_appointment_time(
        &data.db,
        appointment_form.date,
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

    println!(
        "appointment_time: {:?}",
        appointment_time.format("%H:%M").to_string()
    );
    println!("numerical_order: {:?}", Some(numerical_order as i32));

    let appointment = Appointment {
        patient_id: appointment_form.patient_id,
        patient_name: Some(appointment_form.patient_name),
        patient_birthday: Some(appointment_form.patient_birthday),
        patient_phone: Some(appointment_form.patient_phone),
        patient_reason: Some(appointment_form.patient_reason),
        speciality_id: Some(appointment_form.speciality_id),
        numerical_order: Some(numerical_order as i32),
        appointment_time: appointment_time.format("%H:%M").to_string(),
        status: Some("Unpaid".to_string()),
        create_at: Some(Utc::now().naive_utc()),
        update_at: Some(Utc::now().naive_utc()),
        date: appointment_form.date,
    };
    let pool = &data.db;
    match appointment::create_appointment(pool, appointment).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Appointment created successfully",
            "data": AppointmentResponse {
                appointment_time: appointment_time.format("%H:%M").to_string(),
                numerical_order: numerical_order as i32
            }
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

#[get("/{speciality_id}")]
pub async fn get_appointments_by_speciality(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    if claims.role != "doctor" && claims.role != "staff" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Doctor or Staff access required"
        }));
    }

    let speciality_id = path.into_inner();
    match appointment::get_appointments_by_speciality(&data.db, speciality_id).await {
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

#[get("/history/self")]
pub async fn get_self_appointments(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    let patient_id = claims.sub.parse::<i32>().unwrap();
    println!("patient_id at get_self_appointments: {:?}", patient_id);

    match appointment::get_appointment_history(&data.db, patient_id).await {
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
