use crate::models::LoginRequest;
use sqlx::{PgPool, Row}; // Make sure to import necessary models

pub async fn get_user_credentials(
    pool: &PgPool,
    login_req: &LoginRequest,
) -> Result<Option<(i32, String, String, Option<i32>)>, sqlx::Error> {
    let is_patient = login_req.login_type == "patient";
    let is_doctor = login_req.login_type == "doctor";
    let is_receptionist = login_req.login_type == "receptionist";
    let is_staff = login_req.login_type == "staff";
    let is_admin = login_req.login_type == "admin";

    let query = if is_patient {
        "SELECT id, password, name, NULL as speciality_id FROM tn_patients WHERE email = $1"
    } else if is_doctor {
        "SELECT id, password, name, speciality_id FROM tn_doctors WHERE email = $1"
    } else if is_receptionist {
        "SELECT id, password, name, NULL as speciality_id FROM tn_receptionist WHERE email = $1"
    } else if is_staff {
        "SELECT id, password, name, NULL as speciality_id FROM tn_staffs WHERE email = $1"
    } else if is_admin {
        "SELECT id, password, name, NULL as speciality_id FROM tn_admins WHERE email = $1"
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

    Ok(row.map(|row| {
        (
            row.get("id"),
            row.get("password"),
            row.get("name"),
            row.get("speciality_id"),
        )
    }))
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
    } else if role == "admin" {
        "INSERT INTO tn_admins (email, password, name) VALUES ($1, $2, $3)"
    } else if role == "patient" {
        "INSERT INTO tn_patients (email, password, name) VALUES ($1, $2, $3)"
    } else if role == "staff" {
        "INSERT INTO tn_staffs (email, password, name) VALUES ($1, $2, $3)"
    } else if role == "admin" {
        "INSERT INTO tn_admins (email, password, name) VALUES ($1, $2, $3)"
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

pub async fn get_role(pool: &PgPool, email: &str) -> Result<String, sqlx::Error> {
    // Check in patients table
    let patient_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM tn_patients WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await?;

    if patient_count > 0 {
        return Ok("patient".to_string());
    }

    // Check in doctors table
    let doctor_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tn_doctors WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;

    if doctor_count > 0 {
        return Ok("doctor".to_string());
    }

    // Check in admins table
    let admin_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tn_admins WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;

    if admin_count > 0 {
        return Ok("admin".to_string());
    }

    // If email not found in any table
    Err(sqlx::Error::RowNotFound)
}

pub async fn get_user_email(pool: &PgPool, user_id: &str) -> Result<Option<String>, sqlx::Error> {
    // Parse the user_id from string to i32
    let user_id = user_id.parse::<i32>().map_err(|_| {
        sqlx::Error::Protocol("Invalid user ID format".into())
    })?;

    // Query to get user email from all possible user tables
    let query = r#"
        SELECT email FROM (
            SELECT email FROM tn_patients WHERE id = $1
            UNION ALL
            SELECT email FROM tn_doctors WHERE id = $1
            UNION ALL
            SELECT email FROM tn_admins WHERE id = $1
        ) AS users
        LIMIT 1
    "#;

    let result = sqlx::query_scalar::<_, String>(query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

// Optional: Add a function to verify current password before allowing changes
pub async fn verify_current_password(
    pool: &PgPool, 
    email: &str, 
    role: &str,
    current_password: &str
) -> Result<bool, sqlx::Error> {
    let table_name = match role {
        "patient" => "tn_patients",
        "doctor" => "tn_doctors", 
        "admin" => "tn_admins",
        _ => return Ok(false),
    };

    let query = format!(
        "SELECT password FROM {} WHERE email = $1",
        table_name
    );

    let stored_hash: Option<String> = sqlx::query_scalar(&query)
        .bind(email)
        .fetch_optional(pool)
        .await?;

    match stored_hash {
        Some(hash) => Ok(bcrypt::verify(current_password, &hash).unwrap_or(false)),
        None => Ok(false),
    }
}