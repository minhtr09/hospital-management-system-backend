use crate::authentication::Claims;
use crate::db::authentication;
use crate::models::{Patient, PatientForm, PatientQuery};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Add new model for employee
#[derive(Serialize, Deserialize)]
pub struct EmployeeForm {
    username: String,
    password: String,
    role: String,
    department: String,
}

// #[post("/admin/employees")]
// async fn create_employee(
//     claims: Claims,
//     form: web::Json<EmployeeForm>,
//     pool: web::Data<PgPool>,
// ) -> HttpResponse {
//     // Verify admin role
//     if claims.role != "admin" {
//         return HttpResponse::Forbidden().json(json!({
//             "error": "Only admin can create employee accounts"
//         }));
//     }

//     match authentication::create_employee(&pool, &form.into_inner()).await {
//         Ok(_) => HttpResponse::Created().json(json!({
//             "message": "Employee account created successfully"
//         })),
//         Err(e) => HttpResponse::InternalServerError().json(json!({
//             "error": e.to_string()
//         })),
//     }
// }

// #[delete("/admin/employees/{id}")]
// async fn delete_employee(
//     claims: Claims,
//     path: web::Path<i32>,
//     pool: web::Data<PgPool>,
// ) -> HttpResponse {
//     // Verify admin role
//     if claims.role != "admin" {
//         return HttpResponse::Forbidden().json(json!({
//             "error": "Only admin can delete employee accounts"
//         }));
//     }

//     let employee_id = path.into_inner();

//     match authentication::delete_employee(&pool, employee_id).await {
//         Ok(_) => HttpResponse::Ok().json(json!({
//             "message": "Employee account deleted successfully"
//         })),
//         Err(e) => HttpResponse::InternalServerError().json(json!({
//             "error": e.to_string()
//         })),
//     }
// }