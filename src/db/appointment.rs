use crate::error::Error;
use crate::models::Appointment;
use sqlx::PgPool;

pub async fn get_appointment(pool: &PgPool, appointment_id: i32) -> Result<Appointment, Error> {
    sqlx::query_as!(
        Appointment,
        "SELECT appointment_id, patient_id, doctor_id, appointment_datetime, status, notes FROM appointment WHERE appointment_id = $1",
        appointment_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound,
        _ => Error::Database(e),
    })
}
