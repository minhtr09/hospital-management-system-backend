use crate::error::Error;
use crate::models::{Service, ServiceCreateForm};
use sqlx::PgPool;

pub async fn get_services(pool: &PgPool) -> Result<Vec<Service>, Error> {
    sqlx::query_as!(
        Service,
        "SELECT id, name, description,image, price FROM tn_services ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database)
}

pub async fn get_service_by_id(pool: &PgPool, id: i32) -> Result<Service, Error> {
    sqlx::query_as!(
        Service,
        "SELECT id, name, description,image, price FROM tn_services WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::Database)
}

pub async fn create_service(pool: &PgPool, service: &ServiceCreateForm) -> Result<i32, Error> {
    let result = sqlx::query!(
        "INSERT INTO tn_services (name, price, description, image) VALUES ($1, $2, $3, $4) RETURNING id",
        service.name,
        service.price,
        service.description,
        service.image,
    )
    .fetch_one(pool)
    .await
    .map_err(Error::Database)?;

    Ok(result.id)
}

pub async fn update_service(
    pool: &PgPool,
    id: i32,
    service: &ServiceCreateForm,
) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE tn_services SET name = $1, price = $2, description = $3, image = $4 WHERE id = $5",
        service.name,
        service.price,
        service.description,
        service.image,
        id
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;

    Ok(())
}

pub async fn delete_service(pool: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM tn_services WHERE id = $1", id)
        .execute(pool)
        .await
        .map_err(Error::Database)?;

    Ok(())
}
