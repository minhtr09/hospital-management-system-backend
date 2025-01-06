use crate::models::{Patient, PatientForm, UpdatePatientForm};
use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn get_patients(
    pool: &PgPool,
    search: Option<String>,
    order_column: Option<String>,
    order_dir: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Patient>, sqlx::Error> {
    let mut query = "SELECT * FROM tn_patients WHERE 1=1".to_string();

    // Add search condition
    if let Some(search_term) = search {
        query.push_str(&format!(
            " AND (name ILIKE '%{}%' OR email ILIKE '%{}%' OR phone ILIKE '%{}%')",
            search_term, search_term, search_term
        ));
    }

    // Add ordering
    let order_column = order_column.unwrap_or_else(|| "id".to_string());
    let order_dir = order_dir.unwrap_or_else(|| "asc".to_string());
    query.push_str(&format!(" ORDER BY {} {}", order_column, order_dir));

    // Add pagination
    if let Some(limit_val) = limit {
        query.push_str(&format!(" LIMIT {}", limit_val));
    }
    if let Some(offset_val) = offset {
        query.push_str(&format!(" OFFSET {}", offset_val));
    }

    let mut patients = sqlx::query_as::<_, Patient>(&query).fetch_all(pool).await?;

    // Set password to null for all patients
    for patient in &mut patients {
        patient.password = None;
    }

    Ok(patients)
}

pub async fn get_patient_by_id(pool: &PgPool, patient_id: &i32) -> Result<Patient, sqlx::Error> {
    let mut patient = sqlx::query_as!(
        Patient,
        "SELECT * FROM tn_patients WHERE id = $1",
        patient_id
    )
    .fetch_one(pool)
    .await?;

    patient.password = None;
    Ok(patient)
}

pub async fn update_patient(
    pool: &PgPool,
    patient: UpdatePatientForm,
    id: i32,
) -> Result<Patient, sqlx::Error> {
    sqlx::query_as!(
        Patient,
        "UPDATE tn_patients SET name = $1, phone = $2, birthday = $3, gender = $4, address = $5, email = $6 WHERE id = $7 RETURNING *",
        patient.name, patient.phone, patient.birthday, patient.gender, patient.address,patient.email, id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_patient_id_by_email(pool: &PgPool, email: String) -> Result<i32, sqlx::Error> {
    let patient_id = sqlx::query_scalar!("SELECT id FROM tn_patients WHERE email = $1", email)
        .fetch_one(pool)
        .await?;
    Ok(patient_id)
}

pub async fn create_patient(pool: &PgPool, patient: PatientForm, create_at: NaiveDateTime, update_at: NaiveDateTime) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        Patient,
        "INSERT INTO tn_patients (phone, name, gender, birthday, address, create_at, update_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        patient.phone, patient.name, patient.gender, patient.birthday, patient.address, create_at, update_at
    )
    .fetch_one(pool)
    .await;
    Ok(())
}

pub async fn get_patient_by_phone(pool: &PgPool, phone: String) -> Result<Patient, sqlx::Error> {
    let patient = sqlx::query_as!(
        Patient,
        "SELECT * FROM tn_patients WHERE phone = $1",
        phone
    )
    .fetch_one(pool)
    .await?;
    Ok(patient)
}
