use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub account_id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub is_active: bool,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub account_type: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub patient_id: i32,
    pub account_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EmergencyContact {
    pub contact_id: i32,
    pub patient_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub relationship: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoleId {
    Doctor = 1,
    Nurse = 2,
    Staff = 3,
    Patient = 4,
    Admin = 5,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub role_id: i32,
    pub role_name: String,
    pub role_type: RoleType,
    pub description: Option<String>,
    pub permission: PermissionType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoleType {
    Doctor,
    Nurse,
    Staff,
    Patient,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PermissionType {
    FullAccess,
    ReadOnly,
    PatientDataAccess,
    AdministrativeAccess,
    PharmacyAccess,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Doctor {
    pub doctor_id: i32,
    pub account_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub specialty: Option<String>,
    pub phone: Option<String>,
    pub role_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Nurse {
    pub nurse_id: i32,
    pub account_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub specialization: Option<String>,
    pub phone: Option<String>,
    pub role_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalStaff {
    pub staff_id: i32,
    pub account_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub role_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Appointment {
    pub appointment_id: i32,
    pub patient_id: Option<i32>,
    pub doctor_id: Option<i32>,
    pub appointment_datetime: NaiveDateTime,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalRecord {
    pub record_id: i32,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub creation_date: NaiveDateTime,
    pub symptoms: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment_plan: Option<String>,
    pub follow_up_instructions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Prescription {
    pub prescription_id: i32,
    pub medical_record_id: i32,
    pub doctor_id: i32,
    pub prescription_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Medication {
    pub medication_id: i32,
    pub name: String,
    pub generic_name: Option<String>,
    pub manufacturer: Option<String>,
    pub dosage_form: Option<String>,
    pub strength: Option<String>,
    pub unit: Option<String>,
    pub price: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PrescriptionDetail {
    pub prescription_id: i32,
    pub medication_id: i32,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub instructions: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ICD {
    pub code: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct HealthcareService {
    pub service_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub payment_id: i32,
    pub patient_id: i32,
    pub amount: Decimal,
    pub payment_date: NaiveDateTime,
    pub payment_method: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserEvaluation {
    pub evaluation_id: i32,
    pub patient_id: i32,
    pub doctor_id: i32,
    pub rating: i32,
    pub feedback: Option<String>,
    pub evaluation_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub asset_id: i32,
    pub name: String,
    pub r#type: Option<String>,
    pub acquisition_date: Option<NaiveDate>,
    pub purchase_price: Option<Decimal>,
    pub current_value: Option<Decimal>,
    pub location: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Maintenance {
    pub maintenance_id: i32,
    pub asset_id: i32,
    pub maintenance_date: NaiveDateTime,
    pub performed_by: i32,
    pub description: Option<String>,
    pub cost: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InpatientAdmission {
    pub admission_id: i32,
    pub patient_id: i32,
    pub admitting_doctor_id: i32,
    pub primary_nurse_id: i32,
    pub admission_datetime: NaiveDateTime,
    pub discharge_datetime: Option<NaiveDateTime>,
    pub room_number: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment_summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MedicalHistory {
    pub history_id: i32,
    pub patient_id: i32,
    pub record_date: NaiveDateTime,
    pub condition: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CarePlan {
    pub plan_id: i32,
    pub patient_id: i32,
    pub created_by_nurse_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub goals: Option<String>,
    pub interventions: Option<String>,
    pub evaluation_criteria: Option<String>,
}
