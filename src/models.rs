use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Service {
    pub id: i32,
    pub name: Option<String>,
    pub image: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceCreateForm {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Room {
    pub id: i32,
    pub name: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Speciality {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Doctor {
    pub id: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub role: Option<String>,
    pub active: Option<i32>,
    pub avatar: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
    pub speciality_id: Option<i32>,
    pub room_id: Option<i32>,
    pub recovery_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DoctorResponse {
    pub id: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub role: Option<String>,
    pub active: Option<i32>,
    pub avatar: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
    pub speciality: Option<String>,
    pub speciality_id: Option<i32>,
    pub room_id: Option<i32>,
    pub recovery_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DoctorAndService {
    pub id: i32,
    pub service_id: i32,
    pub doctor_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub id: i32,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i32>,
    pub birthday: Option<String>,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientForm {
    pub phone: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i32>,
    pub birthday: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: i32,
    pub service_id: i32,
    pub patient_id: i32,
    pub booking_name: Option<String>,
    pub booking_phone: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i32>,
    pub birthday: Option<String>,
    pub address: Option<String>,
    pub reason: Option<String>,
    pub appointment_date: String,
    pub appointment_time: String,
    pub status: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct BookingPhoto {
    pub id: i32,
    pub url: String,
    pub booking_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Treatment {
    pub id: i32,
    pub appointment_id: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub times: Option<i32>,
    pub purpose: Option<String>,
    pub instruction: Option<String>,
    pub repeat_days: Option<String>,
    pub repeat_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Appointment {
    pub id: Option<i32>, // Đổi thành Option vì khi tạo mới sẽ chưa có id
    pub patient_id: i32,
    pub patient_name: Option<String>,
    pub patient_birthday: Option<String>,
    pub patient_reason: Option<String>,
    pub speciality_id: Option<i32>,
    pub patient_phone: Option<String>,
    pub numerical_order: Option<i32>,
    pub appointment_time: String,
    pub status: Option<String>,
    pub treatment_status: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct VitalSign {
    pub id: i32,
    pub medical_record_id: Option<i32>,
    pub temperature: Option<i32>,
    pub blood_pressure_systolic: Option<i32>,
    pub blood_pressure_diastolic: Option<i32>,
    pub heart_rate: Option<i32>,
    pub spo2: Option<i32>,
    pub weight: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentCreateForm {
    pub patient_id: i32,
    pub patient_name: String,
    pub patient_birthday: String,
    pub patient_phone: String,
    pub patient_reason: String,
    pub speciality_id: i32,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: i32,
    pub message: String,
    pub record_id: i32,
    pub record_type: String,
    pub patient_id: i32,
    pub is_read: Option<i32>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AppointmentRecord {
    pub id: i32,
    pub appointment_id: i32,
    pub reason: Option<String>,
    pub description: Option<String>,
    pub status_before: Option<String>,
    pub status_after: Option<String>,
    pub create_at: Option<NaiveDateTime>,
    pub update_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Drug {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub login_type: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user_data: Option<UserData>, // Add user data to response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TokenData>,
}

#[derive(Serialize)]
pub struct AppointmentResponse {
    pub appointment_time: String,
    pub numerical_order: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AppointmentHistoryResponse {
    pub id: i32,
    pub appointment_time: Option<String>,
    pub date: Option<NaiveDate>,
    pub numerical_order: Option<i32>,
    pub status: Option<String>,
    pub speciality_name: Option<String>,
}

#[derive(Serialize)]
pub struct TokenData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: String,
    pub speciality_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct PasswordResetConfirm {
    pub token: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct PatientQuery {
    pub search: Option<String>,
    #[serde(rename = "order[dir]")]
    pub order_dir: Option<String>,
    #[serde(rename = "order[column]")]
    pub order_column: Option<String>,
    pub length: Option<i32>,
    pub start: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdatePatientForm {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<String>,
    pub gender: Option<i32>,
    pub address: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalRecordResponse {
    pub id: i32,
    pub appointment_id: Option<i32>,
    pub payment_status: Option<i32>,
    pub date: Option<NaiveDate>,
    pub patient_id: Option<i32>,
    pub doctor_name: Option<String>,
    pub diagnosis: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalRecord {
    pub id: Option<i32>,
    pub appointment_id: Option<i32>,
    pub payment_status: Option<i32>,
    pub patient_id: Option<i32>,
    pub doctor_id: Option<i32>,
    pub diagnosis: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i32,
    pub medical_record_id: Option<i32>,
    pub time: Option<NaiveDateTime>,
    pub total_price: Option<i32>,
    pub service_ids: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Medicine {
    pub id: i32,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub manufacture_date: Option<NaiveDateTime>,
    pub expiry_date: Option<NaiveDateTime>,
    pub side_effects: Option<String>,
    pub dosage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicineCreateForm {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub manufacture_date: Option<NaiveDateTime>,
    pub expiry_date: Option<NaiveDateTime>,
    pub side_effects: Option<String>,
    pub dosage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicineOfPrescription {
    pub id: i32,
    pub medical_record_id: Option<i32>,
    pub medicine_ids: Option<Vec<i32>>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTreatmentStatusRequest {
    pub treatment_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub speciality_id: Option<i32>, // Optional since only doctors have this
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InvoiceResponse {
    pub id: i32,
    pub medical_record_id: Option<i32>,
    pub time: Option<NaiveDateTime>,
    pub total_price: Option<i32>,
    pub service_names: Option<Vec<String>>,
    pub service_prices: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub email: String,
}
