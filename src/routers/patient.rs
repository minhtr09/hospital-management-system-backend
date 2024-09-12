use crate::db::{doctor, patient};
use crate::{error::Error, models::Patient};
use sqlx::PgPool;
use warp::{Filter, Rejection, Reply};

pub fn patient_routes(
    pool: PgPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_patient(pool.clone()).or(create_patient(pool))
}

fn get_patient(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "patients" / i32)
        .and(warp::get())
        .and(with_db(pool))
        .and_then(handle_get_patient)
}

fn create_patient(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "patients")
        .and(warp::post())
        .and(json_body())
        .and(with_db(pool))
        .and_then(handle_create_patient)
}

fn with_db(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn json_body() -> impl Filter<Extract = (Patient,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn handle_get_patient(patient_id: i32, pool: PgPool) -> Result<impl Reply, Rejection> {
    let patient = patient::get_patient(&pool, patient_id)
        .await
        .map_err(|e| warp::reject::custom(e))?;
    Ok(warp::reply::json(&patient))
}

async fn handle_create_patient(patient: Patient, pool: PgPool) -> Result<impl Reply, Rejection> {
    patient::create_patient(&pool, &patient)
        .await
        .map_err(|e| warp::reject::custom(e))?;
    Ok(warp::reply::with_status(
        "Patient created successfully",
        warp::http::StatusCode::CREATED,
    ))
}

// pub fn hospital_routes(
//     pool: PgPool,
// ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     patient_routes(pool.clone())
//         .or(doctor_routes(pool.clone()))
//         .or(appointment_routes(pool.clone()))
//         .or(department_routes(pool))
// }

// fn doctor_routes(
//     pool: PgPool,
// ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     get_doctor(pool.clone())
//         .or(create_doctor(pool.clone()))
//         .or(update_doctor(pool.clone()))
//         .or(delete_doctor(pool))
// }

// fn appointment_routes(
//     pool: PgPool,
// ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     get_appointment(pool.clone())
//         .or(create_appointment(pool.clone()))
//         .or(update_appointment(pool.clone()))
//         .or(delete_appointment(pool))
// }

// fn department_routes(
//     pool: PgPool,
// ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     get_department(pool.clone())
//         .or(create_department(pool.clone()))
//         .or(update_department(pool.clone()))
//         .or(delete_department(pool))
// }

// // Define CRUD operations for doctors, appointments, and departments
// // (similar to the existing patient functions)

// // Add more route handlers for other entities and operations
