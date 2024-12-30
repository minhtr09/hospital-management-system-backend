use crate::error::Error;
use crate::models::{Specialty};
use sqlx::PgPool;

pub async fn get_specialties(pool: &PgPool) -> Result<Vec<Specialty>, Error> {
    sqlx::query_as!(
        Specialty,
        "SELECT id, name, description, image FROM tn_specialties"
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database)
}

pub async fn get_specialty(pool: &PgPool, id: i32) -> Result<Specialty, Error> {
    sqlx::query_as!(
        Specialty,
        "SELECT id, name, description, image FROM tn_specialties WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound,
        _ => Error::Database(e),
    })
}

pub async fn create_specialty(pool: &PgPool, specialty: &Specialty) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tn_specialties (name, description, image) VALUES ($1, $2, $3)",
        specialty.name,
        specialty.description,
        specialty.image
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;
    Ok(())
}

pub async fn update_specialty(pool: &PgPool, id: i32, specialty: &Specialty) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE tn_specialties SET name = $1, description = $2, image = $3 WHERE id = $4",
        specialty.name,
        specialty.description,
        specialty.image,
        id
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;
    Ok(())
}

pub async fn delete_specialty(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM tn_specialties WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
    Ok(())
}


