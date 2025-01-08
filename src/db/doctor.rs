use crate::error::Error;
use crate::models::{Doctor, DoctorResponse};
use sqlx::PgPool;

pub async fn get_doctors(
    pool: &PgPool,
    search: Option<String>,
    order_column: Option<String>,
    order_dir: Option<String>,
) -> Result<Vec<Doctor>, sqlx::Error> {
    let mut query = "SELECT * FROM tn_doctors WHERE 1=1".to_string();

    // Add search condition
    if let Some(search_term) = search {
        query.push_str(&format!(
            " AND (name ILIKE '%{}%' OR email ILIKE '%{}%' OR phone ILIKE '%{}%' )",
            search_term, search_term, search_term
        ));
    }

    // Add ordering
    let order_column = order_column.unwrap_or_else(|| "id".to_string());
    let order_dir = order_dir.unwrap_or_else(|| "asc".to_string());
    query.push_str(&format!(" ORDER BY {} {}", order_column, order_dir));

    let doctors = sqlx::query_as::<_, Doctor>(&query).fetch_all(pool).await?;
    Ok(doctors)
}

pub async fn get_doctor_by_id(pool: &PgPool, id: &i32) -> Result<Doctor, Error> {
    sqlx::query_as!(Doctor, "SELECT * FROM tn_doctors WHERE id = $1", id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::Database(e),
        })
}

pub async fn create_doctor(pool: &PgPool, doctor: &Doctor) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tn_doctors (email, phone, name, description, role, avatar, speciality_id, room_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        doctor.email,
        doctor.phone,
        doctor.name,
        doctor.description,
        doctor.role,
        doctor.avatar,
        doctor.speciality_id,
        doctor.room_id
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;
    Ok(())
}

pub async fn update_doctor(pool: &PgPool, email: String, doctor: &Doctor) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE tn_doctors SET phone = $1, name = $2, description = $3, role = $4, avatar = $5, speciality_id = $6, room_id = $7 WHERE email = $8",
        doctor.phone,
        doctor.name,
        doctor.description,
        doctor.role,
        doctor.avatar,
        doctor.speciality_id,
        doctor.room_id,
        email
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;
    Ok(())
}

pub async fn delete_doctor(pool: &PgPool, id: &i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM tn_doctors WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
    Ok(())
}
