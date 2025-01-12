use crate::error::Error;
use crate::models::{MedicalRecord, MedicalRecordResponse, VitalSign};
use sqlx::PgPool;

enum PaymentStatus {
    Paid = 1,
    Unpaid = 0,
}

pub async fn create(pool: &PgPool, record: &MedicalRecord) -> Result<i32, Error> {
    let result = sqlx::query!(
        "INSERT INTO tn_medical_records (appointment_id, payment_status, patient_id, doctor_id, diagnosis) 
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
        record.appointment_id,
        record.payment_status,
        record.patient_id,
        record.doctor_id,
        record.diagnosis
    )
    .fetch_one(pool)
    .await
    .map_err(Error::Database)?;

    Ok(result.id)
}

pub async fn get_by_patient_id(
    pool: &PgPool,
    patient_id: i32,
) -> Result<Vec<MedicalRecordResponse>, Error> {
    sqlx::query_as!(
        MedicalRecordResponse,
        "SELECT mr.id, mr.appointment_id, mr.payment_status, mr.patient_id, 
        mr.diagnosis, d.name as doctor_name, a.date
        FROM tn_medical_records mr
        JOIN tn_doctors d ON mr.doctor_id = d.id
        JOIN tn_appointments a ON mr.appointment_id = a.id
        WHERE mr.patient_id = $1",
        patient_id
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database)
}

pub async fn update_payment_status(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE tn_medical_records 
         SET payment_status = $1
         WHERE id = $2",
        PaymentStatus::Paid as i32,
        id
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;

    Ok(())
}

pub async fn get_vital_signs(
    pool: &PgPool,
    medical_record_id: i32,
) -> Result<Vec<VitalSign>, Error> {
    sqlx::query_as!(
        VitalSign,
        "SELECT * FROM tn_vital_signs WHERE medical_record_id = $1",
        medical_record_id
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database)
}

pub async fn create_vital_sign(pool: &PgPool, vital_sign: &VitalSign) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tn_vital_signs (medical_record_id, temperature, blood_pressure_systolic, blood_pressure_diastolic, heart_rate, spo2, weight, height) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        vital_sign.medical_record_id,
        vital_sign.temperature,
        vital_sign.blood_pressure_systolic,
        vital_sign.blood_pressure_diastolic,
        vital_sign.heart_rate,
        vital_sign.spo2,
        vital_sign.weight,
        vital_sign.height
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;

    Ok(())
}

pub async fn update_diagnosis(pool: &PgPool, id: i32, diagnosis: &str) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE tn_medical_records 
         SET diagnosis = $1
         WHERE id = $2",
        diagnosis,
        id
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;

    Ok(())
}

pub async fn get_by_appointment_id(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<MedicalRecordResponse, Error> {
    let record = sqlx::query_as!(
        MedicalRecordResponse,
        "SELECT mr.id, mr.appointment_id, mr.payment_status, mr.patient_id, 
        mr.diagnosis, d.name as doctor_name, a.date
        FROM tn_medical_records mr
        JOIN tn_doctors d ON mr.doctor_id = d.id
        JOIN tn_appointments a ON mr.appointment_id = a.id
        WHERE mr.appointment_id = $1",
        appointment_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Error::Database)?;

    match record {
        Some(record) => Ok(record),
        None => Err(Error::NotFound),
    }
}

pub async fn is_medical_record_exist(
    pool: &PgPool,
    appointment_id: i32,
) -> Result<(bool, Option<i32>), Error> {
    let record = sqlx::query!(
        "SELECT COUNT(*) as count, id FROM tn_medical_records WHERE appointment_id = $1 GROUP BY id",
        appointment_id
    )
    .fetch_optional(pool)
    .await
    .map_err(Error::Database)?;

    match record {
        Some(r) => Ok((r.count > Some(0), Some(r.id))),
        None => Ok((false, None)),
    }
}

