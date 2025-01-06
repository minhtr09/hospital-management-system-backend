use crate::models::{Medicine, MedicineCreateForm};
use crate::error::Error;
use sqlx::PgPool;

pub async fn get_medicines(pool: &PgPool) -> Result<Vec<Medicine>, Error> {
    sqlx::query_as!(
        Medicine,
        "SELECT id, name, price, unit, description, manufacture_date, expiry_date, side_effects, dosage FROM tn_medicine"
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database) 
}


pub async fn get_medicine_by_id(pool: &PgPool, id: i32) -> Result<Medicine, sqlx::Error> {
    sqlx::query_as!(
        Medicine,
        "SELECT * FROM tn_medicine WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_medicine(pool: &PgPool, medicine: &MedicineCreateForm) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO tn_medicine (name, price, unit, description, manufacture_date, expiry_date, side_effects, dosage) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        medicine.name,
        medicine.price,
        medicine.unit,
        medicine.description,
        medicine.manufacture_date,
        medicine.expiry_date,
        medicine.side_effects,
        medicine.dosage
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_medicine(pool: &PgPool, id: i32, medicine: &Medicine) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE tn_medicine SET 
         name = $1, price = $2, unit = $3, description = $4, 
         manufacture_date = $5, expiry_date = $6, side_effects = $7, dosage = $8
         WHERE id = $9",
        medicine.name,
        medicine.price,
        medicine.unit,
        medicine.description,
        medicine.manufacture_date,
        medicine.expiry_date,
        medicine.side_effects,
        medicine.dosage,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_medicine(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM tn_medicine WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}