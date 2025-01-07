use crate::error::Error;
use crate::models::MedicalRecord;
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
) -> Result<Vec<MedicalRecord>, Error> {
    sqlx::query_as!(
        MedicalRecord,
        "SELECT id, appointment_id, payment_status, patient_id, doctor_id, diagnosis 
         FROM tn_medical_records WHERE patient_id = $1",
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
