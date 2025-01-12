use crate::authentication::Claims;
use crate::db::{appointment, patient};
use crate::models::{
    Appointment, AppointmentCreateForm, AppointmentResponse, Patient, UpdateStatusRequest,
    UpdateTreatmentStatusRequest,
};
use actix_web::{get, post, put, web, HttpResponse};
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
        id: None,
        patient_id: appointment_form.patient_id,
        patient_name: Some(appointment_form.patient_name),
        patient_birthday: Some(appointment_form.patient_birthday),
        patient_phone: Some(appointment_form.patient_phone),
        patient_reason: Some(appointment_form.patient_reason),
        speciality_id: Some(appointment_form.speciality_id),
        numerical_order: Some(numerical_order as i32),
        appointment_time: appointment_time.format("%H:%M").to_string(),
        status: Some("Unpaid".to_string()),
        treatment_status: Some("scheduled".to_string()),
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

#[get("/specialty/{specialityId}")]
pub async fn get_appointments_by_specialty(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    if claims.role != "doctor" {
        return HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Doctor access required"
        }));
    }

    let specialty_id = path.into_inner();
    let query = "SELECT DISTINCT a.* FROM tn_appointments a 
                INNER JOIN tn_doctors d ON d.speciality_id = a.speciality_id 
                WHERE d.speciality_id = $1
                ORDER BY a.create_at";

    let appointments = match sqlx::query_as::<_, Appointment>(query)
        .bind(specialty_id)
        .fetch_all(&data.db)
        .await
    {
        Ok(appointments) => appointments,
        Err(e) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": format!("Failed to fetch appointments: {}", e)
            }));
        }
    };

    HttpResponse::Ok().json(json!({
        "success": true,
        "data": appointments,
        "message": "Appointments fetched successfully"
    }))
}

#[put("/status/{id}")]
pub async fn update_appointment_status(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
    body: web::Json<UpdateStatusRequest>,
) -> HttpResponse {
    // if claims.role != "doctor" {
    //     return HttpResponse::Forbidden().json(json!({
    //         "success": false,
    //         "message": "Doctor access required"
    //     }));
    // }
    let new_status = body.into_inner().status;
    let appointment_id = path.into_inner();

    match appointment::update_appointment_status(&data.db, appointment_id, new_status).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Status updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to update status: {}", e)
        })),
    }
}

#[put("/treatment-status/{id}")]
pub async fn update_appointment_treatment_status(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    body: web::Json<UpdateTreatmentStatusRequest>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // if claims.role != "doctor" {
    //     return HttpResponse::Forbidden().json(json!({
    //         "success": false,
    //         "message": "Doctor access required"
    //     }));
    // }

    let appointment_id = path.into_inner();

    match appointment::update_appointment_treatment_status(
        &data.db,
        appointment_id,
        body.treatment_status.clone(),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Treatment status updated successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to update treatment status: {}", e)
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
