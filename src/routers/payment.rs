use crate::authentication::Claims;
use crate::db::payment;
use crate::models::Invoice;
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;

#[get("/invoices/{id}")]
pub async fn get_invoices_of_medical_record(
    data: web::Data<crate::AppState>,
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
) -> HttpResponse {
    // Check if user has receptionist role
    check_receptionist(&claims);

    match payment::get_invoices_of_medical_record(&data.db, path.into_inner()).await {
        Ok(invoices) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": invoices,
            "message": "Invoices retrieved successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to retrieve invoices: {}", e)
        })),
    }
}

#[post("/invoices")]
pub async fn create_invoice(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    body: web::Json<Invoice>,
) -> HttpResponse {
    // Check if user has receptionist role
    check_receptionist(&claims.into_inner()).unwrap();
    match payment::create_invoice_of_medical_record(&data.db, body.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Invoice created successfully"
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": format!("Failed to create invoice: {}", e)
        })),
    }
}


fn check_receptionist(claims: &Claims) -> Result<(), HttpResponse> {
    if claims.role != "receptionist" {
        return Err(HttpResponse::Forbidden().json(json!({
            "success": false,
            "message": "Receptionist access required"
        })));
    }
    Ok(())
}
