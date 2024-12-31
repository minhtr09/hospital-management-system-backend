use crate::error::Error;
use crate::models::{Speciality};
use sqlx::PgPool;

pub async fn get_specialties(pool: &PgPool) -> Result<Vec<Speciality>, Error> {
    sqlx::query_as!(
        Speciality,
        "SELECT id, name, description, image FROM tn_specialities"
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database)
}

pub async fn get_specialty(pool: &PgPool, id: i32) -> Result<Speciality, Error> {
    sqlx::query_as!(
        Speciality,
        "SELECT id, name, description, image FROM tn_specialities WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound,
        _ => Error::Database(e),
    })
}

pub async fn create_specialty(pool: &PgPool, speciality: &Speciality) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO tn_specialities (name, description, image) VALUES ($1, $2, $3)",
        speciality.name,
        speciality.description,
        speciality.image
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;
    Ok(())
}

pub async fn update_specialty(pool: &PgPool, id: i32, speciality: &Speciality) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE tn_specialities SET name = $1, description = $2, image = $3 WHERE id = $4",
        speciality.name,
        speciality.description,
        speciality.image,
        id
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;
    Ok(())
}

pub async fn delete_specialty(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM tn_specialities WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
    Ok(())
}


