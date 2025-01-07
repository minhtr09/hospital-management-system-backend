use crate::models::LoginRequest;
use sqlx::{PgPool, Row}; // Make sure to import necessary models

pub async fn get_user_credentials(
    pool: &PgPool,
    login_req: &LoginRequest,
) -> Result<Option<(i32, String, String)>, sqlx::Error> {
    let is_patient = login_req.login_type == "patient";
    let is_doctor = login_req.login_type == "doctor";
    let is_receptionist = login_req.login_type == "receptionist";
    let is_staff = login_req.login_type == "staff";

    let query = if is_patient {
        "SELECT id, password, name FROM tn_patients WHERE email = $1"
    } else if is_doctor {
        "SELECT id, password, name, speciality_id FROM tn_doctors WHERE email = $1"
    } else if is_receptionist {
        "SELECT id, password, name FROM tn_receptionist WHERE email = $1"
    } else if is_staff {
        "SELECT id, password, name FROM tn_staffs WHERE email = $1"
    } else {
        return Err(sqlx::Error::RowNotFound);
    };

    let row = sqlx::query(query)
        .bind(&login_req.email)
        .fetch_optional(pool)
        .await?;

    if let Some(r) = &row {
        println!(
            "Found user: id={}, name={}",
            r.get::<i32, _>("id"),
            r.get::<String, _>("name")
        );
    }

    Ok(row.map(|row| (row.get("id"), row.get("password"), row.get("name"))))
}
pub async fn create_user(
    pool: &PgPool,
    email: &str,
    hashed_password: &str,
    name: &str,
    role: &str,
) -> Result<(), sqlx::Error> {
    let query = if role == "doctor" {
        "INSERT INTO tn_doctors (email, password, name) VALUES ($1, $2, $3)"
    } else if role == "receptionist" {
        "INSERT INTO tn_receptionist (email, password, name) VALUES ($1, $2, $3)"
    } else if role == "patient" {
        "INSERT INTO tn_patients (email, password, name) VALUES ($1, $2, $3)"
    } else if role == "staff" {
        "INSERT INTO tn_staffs (email, password, name) VALUES ($1, $2, $3)"
    } else {
        return Err(sqlx::Error::RowNotFound);
    };

    sqlx::query(query)
        .bind(email)
        .bind(hashed_password)
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_password(
    pool: &PgPool,
    role: &str,
    email: &str,
    new_password: &str,
) -> Result<bool, sqlx::Error> {
    let is_patient = role == "patient";
    let query = if is_patient {
        "UPDATE tn_patients SET password = $1 WHERE email = $2 RETURNING id"
    } else {
        "UPDATE tn_doctors SET password = $1 WHERE email = $2 RETURNING id"
    };

    let result = sqlx::query(query)
        .bind(new_password)
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(result.is_some())
}

pub async fn user_exists(pool: &PgPool, email: &str, role: &str) -> Result<bool, sqlx::Error> {
    let is_patient = role == "patient";
    let query = if is_patient {
        "SELECT COUNT(*) FROM tn_patients WHERE email = $1"
    } else {
        "SELECT COUNT(*) FROM tn_doctors WHERE email = $1"
    };

    let count: i64 = sqlx::query_scalar(query)
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(count > 0)
}


