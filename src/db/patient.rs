use crate::error::Error;
use crate::models::{
    Appointment, Asset, Doctor, InpatientAdmission, Maintenance, MedicalRecord, Medication,
    Patient, Payment, Prescription, Role, UserEvaluation,
};
use sqlx::PgPool;

pub async fn get_patient(pool: &PgPool, patient_id: i32) -> Result<Patient, Error> {
    sqlx::query_as!(
        Patient,
        "SELECT patient_id, account_id, first_name, last_name, date_of_birth, gender, address, phone FROM patient WHERE patient_id = $1",
        patient_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound,
        _ => Error::Database(e),
    })
}

pub async fn create_patient(pool: &PgPool, patient: &Patient) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO patient (account_id, first_name, last_name, date_of_birth, gender, address, phone) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        patient.account_id,
        patient.first_name,
        patient.last_name,
        patient.date_of_birth,
        patient.gender,
        patient.address,
        patient.phone
    )
    .execute(pool)
    .await
    .map_err(Error::Database)?;

    Ok(())
}

// pub async fn create_appointment(pool: &PgPool, appointment: &Appointment) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO appointment (appointment_id, patient_id, doctor_id, appointment_date, appointment_time, appointment_status)
//          VALUES ($1, $2, $3, $4, $5, $6)",
//         appointment.appointment_id,
//         appointment.patient_id,
//         appointment.doctor_id,
//         appointment.appointment_date,
//         appointment.appointment_time,
//         appointment.appointment_status
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_medical_record(pool: &PgPool, record_id: i32) -> Result<MedicalRecord, Error> {
//     sqlx::query_as!(
//         MedicalRecord,
//         "SELECT * FROM medical_record WHERE record_id = $1",
//         record_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_medical_record(pool: &PgPool, record: &MedicalRecord) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO medical_record (record_id, patient_id, creation_date, symptoms, diagnosis, treatment_method, patient_status)
//          VALUES ($1, $2, $3, $4, $5, $6, $7)",
//         record.record_id,
//         record.patient_id,
//         record.creation_date,
//         record.symptoms,
//         record.diagnosis,
//         record.treatment_method,
//         record.patient_status
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_prescription(pool: &PgPool, prescription_id: i32) -> Result<Prescription, Error> {
//     sqlx::query_as!(
//         Prescription,
//         "SELECT * FROM prescription WHERE prescription_id = $1",
//         prescription_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_prescription(pool: &PgPool, prescription: &Prescription) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO prescription (prescription_id, patient_id, doctor_id, prescription_date)
//          VALUES ($1, $2, $3, $4)",
//         prescription.prescription_id,
//         prescription.patient_id,
//         prescription.doctor_id,
//         prescription.prescription_date
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_medication(pool: &PgPool, medication_id: i32) -> Result<Medication, Error> {
//     sqlx::query_as!(
//         Medication,
//         "SELECT * FROM medication WHERE medication_id = $1",
//         medication_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_medication(pool: &PgPool, medication: &Medication) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO medication (medication_id, medication_name, unit, route_of_administration, medication_price)
//          VALUES ($1, $2, $3, $4, $5)",
//         medication.medication_id,
//         medication.medication_name,
//         medication.unit,
//         medication.route_of_administration,
//         medication.medication_price
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_payment(pool: &PgPool, payment_id: i32) -> Result<Payment, Error> {
//     sqlx::query_as!(
//         Payment,
//         "SELECT * FROM payment WHERE payment_id = $1",
//         payment_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_payment(pool: &PgPool, payment: &Payment) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO payment (payment_id, patient_id, payment_date, amount, payment_method)
//          VALUES ($1, $2, $3, $4, $5)",
//         payment.payment_id,
//         payment.patient_id,
//         payment.payment_date,
//         payment.amount,
//         payment.payment_method
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_user_evaluation(pool: &PgPool, evaluation_id: i32) -> Result<UserEvaluation, Error> {
//     sqlx::query_as!(
//         UserEvaluation,
//         "SELECT * FROM user_evaluation WHERE evaluation_id = $1",
//         evaluation_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_user_evaluation(pool: &PgPool, evaluation: &UserEvaluation) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO user_evaluation (evaluation_id, patient_id, rating, feedback, evaluation_date)
//          VALUES ($1, $2, $3, $4, $5)",
//         evaluation.evaluation_id,
//         evaluation.patient_id,
//         evaluation.rating,
//         evaluation.feedback,
//         evaluation.evaluation_date
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_asset(pool: &PgPool, asset_id: i32) -> Result<Asset, Error> {
//     sqlx::query_as!(
//         Asset,
//         "SELECT * FROM asset WHERE asset_id = $1",
//         asset_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_asset(pool: &PgPool, asset: &Asset) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO asset (asset_id, asset_name, start_date, years_in_use, conventional_price, condition, current_room)
//          VALUES ($1, $2, $3, $4, $5, $6, $7)",
//         asset.asset_id,
//         asset.asset_name,
//         asset.start_date,
//         asset.years_in_use,
//         asset.conventional_price,
//         asset.condition,
//         asset.current_room
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_maintenance(pool: &PgPool, maintenance_id: i32) -> Result<Maintenance, Error> {
//     sqlx::query_as!(
//         Maintenance,
//         "SELECT * FROM maintenance WHERE maintenance_id = $1",
//         maintenance_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_maintenance(pool: &PgPool, maintenance: &Maintenance) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO maintenance (maintenance_id, asset_id, maintenance_date, condition, repair_work, maintenance_cost)
//          VALUES ($1, $2, $3, $4, $5, $6)",
//         maintenance.maintenance_id,
//         maintenance.asset_id,
//         maintenance.maintenance_date,
//         maintenance.condition,
//         maintenance.repair_work,
//         maintenance.maintenance_cost
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }

// pub async fn get_inpatient_admission(pool: &PgPool, admission_id: i32) -> Result<InpatientAdmission, Error> {
//     sqlx::query_as!(
//         InpatientAdmission,
//         "SELECT * FROM inpatient_admission WHERE admission_id = $1",
//         admission_id
//     )
//     .fetch_one(pool)
//     .await
//     .map_err(|e| match e {
//         sqlx::Error::RowNotFound => Error::NotFound,
//         _ => Error::Database(e),
//     })
// }

// pub async fn create_inpatient_admission(pool: &PgPool, admission: &InpatientAdmission) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO inpatient_admission (admission_id, patient_id, admission_date, department, room_number, discharge_date, discharge_condition)
//          VALUES ($1, $2, $3, $4, $5, $6, $7)",
//         admission.admission_id,
//         admission.patient_id,
//         admission.admission_date,
//         admission.department,
//         admission.room_number,
//         admission.discharge_date,
//         admission.discharge_condition
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }
