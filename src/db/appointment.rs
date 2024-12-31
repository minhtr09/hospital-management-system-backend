use crate::error::Error;
use crate::models::Appointment;
use chrono::{Duration, NaiveDate, NaiveTime};
use sqlx::PgPool;

pub async fn get_appointments_of_patient(
    pool: &PgPool,
    patient_id: i32,
) -> Result<Vec<Appointment>, Error> {
    let query = "SELECT * FROM tn_appointments WHERE patient_id = $1";
    sqlx::query_as::<_, Appointment>(&query)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)
}

pub async fn get_appointments_of_doctor(
    pool: &PgPool,
    doctor_id: i32,
) -> Result<Vec<Appointment>, Error> {
    let query = "SELECT * FROM tn_appointments WHERE doctor_id = $1";
    sqlx::query_as::<_, Appointment>(&query)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)
}

pub async fn create_appointment(pool: &PgPool, appointment: Appointment) -> Result<(), Error> {
    let query = "INSERT INTO tn_appointments (patient_id, patient_name, patient_birthday, patient_phone, patient_reason, speciality_id, date, numerical_order, appointment_time, status, create_at, update_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)";
    sqlx::query(query)
        .bind(appointment.patient_id)
        .bind(appointment.patient_name)
        .bind(appointment.patient_birthday)
        .bind(appointment.patient_phone)
        .bind(appointment.patient_reason)
        .bind(appointment.speciality_id)
        .bind(appointment.date)
        .bind(appointment.numerical_order)
        .bind(appointment.appointment_time)
        .bind(appointment.status)
        .bind(appointment.create_at)
        .bind(appointment.update_at)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
    Ok(())
}

pub async fn update_appointment_status(
    pool: &PgPool,
    id: i32,
    status: String,
) -> Result<(), Error> {
    let query: &str = "UPDATE tn_appointments SET status = $1 WHERE id = $2";
    sqlx::query(query)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
    Ok(())
}
pub async fn update_appointment_time(
    pool: &PgPool,
    id: i32,
    time: String,
    order: i32,
) -> Result<(), Error> {
    let query: &str =
        "UPDATE tn_appointments SET appointment_time = $1, numerical_order = $2 WHERE id = $3";
    sqlx::query(query)
        .bind(time)
        .bind(order)
        .bind(id)
        .execute(pool)
        .await
        .map_err(Error::Database)?;

    Ok(())
}

pub async fn calculate_appointment_time(
    pool: &PgPool,
    date: Option<NaiveDate>,
    speciality_id: Option<i32>,
) -> Result<(i64, NaiveTime), Error> {
    let numerical_order = sqlx::query_scalar!(
        "SELECT COUNT(*) + 1 FROM tn_appointments WHERE date = $1 AND speciality_id = $2",
        date,
        speciality_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::Database)?
    .unwrap_or(1);

    let base_time = NaiveTime::parse_from_str("07:00", "%H:%M").unwrap();
    let minutes_to_add = (numerical_order - 1) * 30;
    let appointment_time = base_time + Duration::minutes(minutes_to_add);

    Ok((numerical_order, appointment_time))
}
