use crate::error::Error;
use crate::models::{Invoice, InvoiceResponse};
use sqlx::PgPool;
pub async fn get_invoices_of_medical_record(
    pool: &PgPool,
    medical_record_id: i32,
) -> Result<Vec<Invoice>, sqlx::Error> {
    sqlx::query_as!(
        Invoice,
        "SELECT id, medical_record_id, time, total_price, service_ids FROM tn_invoices WHERE medical_record_id = $1",
        medical_record_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_invoice(pool: &PgPool, invoice: Invoice) -> Result<(), Error> {
    sqlx::query_as!(
        Invoice,
        "INSERT INTO tn_invoices (medical_record_id, time, total_price, service_ids) 
         VALUES ($1, $2, $3, $4) 
         RETURNING id, medical_record_id, time, total_price, service_ids",
        invoice.medical_record_id,
        invoice.time,
        invoice.total_price,
        invoice.service_ids.as_deref(),
    )
    .fetch_one(pool)
    .await
    .map_err(Error::Database)?;

    Ok(())
}

pub async fn get_invoices_of_user(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<InvoiceResponse>, Error> {
    let invoices = sqlx::query_as!(
        InvoiceResponse,
        "SELECT 
            i.id,
            i.medical_record_id,
            i.time,
            i.total_price,
            array_agg(s.name) as service_names,
            array_agg(s.price) as service_prices
        FROM tn_invoices i
        LEFT JOIN tn_medical_records mr ON i.medical_record_id = mr.id
        LEFT JOIN tn_services s ON s.id = ANY(i.service_ids)
        WHERE mr.patient_id = $1
        GROUP BY i.id, i.medical_record_id, i.time, i.total_price",
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Database)?;
    println!("{:?}", invoices);
    Ok(invoices)
}
