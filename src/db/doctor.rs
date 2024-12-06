use crate::error::Error;
use crate::models::DoctorResponse;
use sqlx::PgPool;

pub async fn get_doctor(pool: &PgPool, email: String) -> Result<DoctorResponse, Error> {
    sqlx::query_as!(
        DoctorResponse,
        "SELECT email, phone, name, description, price, role, avatar, create_at, update_at, specialty_id, room_id FROM tn_doctors WHERE email = $1",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound,
        _ => Error::Database(e),
    })
}

// pub async fn create_doctor(pool: &PgPool, doctor: &Doctor) -> Result<(), Error> {
//     sqlx::query!(
//         "INSERT INTO doctor (account_id, first_name, last_name, specialty, phone, role_id)
//          VALUES ($1, $2, $3, $4, $5, $6)",
//         doctor.account_id,
//         doctor.first_name,
//         doctor.last_name,
//         doctor.specialty,
//         doctor.phone,
//         RoleId::Doctor as i32
//     )
//     .execute(pool)
//     .await
//     .map_err(Error::Database)?;

//     Ok(())
// }
