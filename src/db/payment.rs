use crate::models::Invoice;
use sqlx::PgPool;

pub async fn get_invoices_of_medical_record(
    pool: &PgPool,
    medical_record_id: i32,
) -> Result<Vec<Invoice>, sqlx::Error> {
    sqlx::query_as!(
        Invoice,
        "SELECT medical_record_id, time, total_price FROM tn_invoices WHERE medical_record_id = $1",
        medical_record_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_invoice_of_medical_record(
    pool: &PgPool,
    invoice: Invoice,
) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        Invoice,
        "INSERT INTO tn_invoices (medical_record_id, time, total_price) VALUES ($1, $2, $3) RETURNING medical_record_id, time, total_price",
        invoice.medical_record_id,
        invoice.time,
        invoice.total_price
    )
    .fetch_one(pool)
    .await;

    Ok(())
}
