use std::ptr::null;

use crate::db::{authentication, doctor, patient};
use crate::models::{
    LoginRequest, LoginResponse, PasswordResetRequest, RegisterRequest, TokenData,
    UpdatePasswordRequest, UserData,
};
use actix_web::{get, post, put, web, HttpResponse};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub role: String,
    pub exp: i64,
}

fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

#[post("/login")]
pub async fn login(
    data: web::Data<crate::AppState>,
    login_req: web::Json<LoginRequest>,
) -> HttpResponse {
    let pool = &data.db;

    println!("login request: {:?}", login_req);

    // Query the database using the authentication module
    let credentials: (i32, String, String, Option<i32>) =
        match authentication::get_user_credentials(pool, &login_req).await {
            Ok(Some(creds)) => creds,
            Ok(None) => {
                return HttpResponse::Unauthorized().json(LoginResponse {
                    success: false,
                    message: "Wrong email or password".to_string(),
                    data: None,
                    user_data: None,
                });
            }
            Err(_) => {
                return HttpResponse::InternalServerError().json(LoginResponse {
                    success: false,
                    message: "Database error".to_string(),
                    data: None,
                    user_data: None,
                });
            }
        };

    // Check if user exists
    let (id, hashed_password, name, speciality_id) = credentials;

    // Verify password
    if !verify_password(&login_req.password, &hashed_password) {
        return HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Invalid credentials".to_string(),
            data: None,
            user_data: None,
        });
    }

    // Generate JWT token
    let claims = Claims {
        sub: id.to_string(),
        name: name.clone(),
        role: login_req.login_type.clone(),
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
    };

    println!("{:?}", claims);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.jwt_secret.as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        data: Some(TokenData {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 86400, // 24 hours in seconds
        }),
        user_data: Some(UserData {
            id,
            name,
            role: login_req.login_type.clone(),
            speciality_id,
        }),
    })
}

#[post("/register")]
pub async fn register(
    data: web::Data<crate::AppState>,
    register_req: web::Json<RegisterRequest>,
) -> HttpResponse {
    // if register_req.role != "patient" {
    //     return HttpResponse::BadRequest().json(json!({
    //         "success": false,
    //         "message": "Only patient is supported for registration"
    //     }));
    // }

    let pool = &data.db;
    // Hash the password
    let hashed_password = match hash(&register_req.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return HttpResponse::InternalServerError().json(LoginResponse {
                success: false,
                message: "Password hashing failed".to_string(),
                data: None,
                user_data: None,
            });
        }
    };

    // Attempt to create the user
    match authentication::create_user(
        pool,
        &register_req.email,
        &hashed_password,
        &register_req.name,
        &register_req.role,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "User registered successfully".to_string(),
            data: None,
            user_data: None,
        }),
        Err(e) => {
            // You might want to handle different error types differently
            HttpResponse::BadRequest().json(LoginResponse {
                success: false,
                message: format!("Registration failed: {}", e),
                data: None,
                user_data: None,
            })
        }
    }
}

#[post("/reset-password")]
pub async fn reset_password(
    data: web::Data<crate::AppState>,
    reset_req: web::Json<PasswordResetRequest>,
) -> HttpResponse {
    let pool = &data.db;

    match authentication::user_exists(pool, &reset_req.email, &reset_req.role).await {
        Ok(true) => (),
        Ok(false) => {
            return HttpResponse::BadRequest().json(json!({
                "success": false,
                "message": "User not found"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to check user existence"
            }));
        }
    }

    // Generate a temporary password
    let temp_password = Alphanumeric.sample_string(&mut rand::thread_rng(), 12);

    // Hash the temporary password
    let hashed_password = match hash(&temp_password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Password generation failed"
            }));
        }
    };

    // Update password in database
    match authentication::update_password(pool, &reset_req.role, &reset_req.email, &hashed_password)
        .await
    {
        Ok(true) => {
            // Send email with new password
            let email = Message::builder()
                .from("your-email@domain.com".parse().unwrap())
                .to(reset_req.email.parse().unwrap())
                .subject("Password Reset")
                .header(ContentType::TEXT_PLAIN)
                .body(format!(
                    "Your new temporary password is: {}\nPlease change it after logging in.",
                    temp_password
                ))
                .unwrap();

            let creds = Credentials::new(
                "minhtr090102@gmail.com".to_string(),
                "zkfc hzzy jfza cxex".to_string(),
            );

            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            match mailer.send(&email) {
                Ok(_) => HttpResponse::Ok().json(json!({
                    "success": true,
                    "message": "New password has been sent to your email"
                })),
                Err(_) => HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": "Failed to send email"
                })),
            }
        }
        Ok(false) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Email not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Failed to reset password"
        })),
    }
}

#[get("/role/{email}")]
pub async fn get_role(data: web::Data<crate::AppState>, email: web::Path<String>) -> HttpResponse {
    let pool = &data.db;
    let email = email.into_inner();
    match authentication::get_role(pool, &email).await {
        Ok(role) => HttpResponse::Ok().json(role),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Failed to get role"
        })),
    }
}

#[put("/update-password")]
pub async fn update_password(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    new_password: web::Json<UpdatePasswordRequest>,
) -> HttpResponse {
    let pool = &data.db;

    // For non-admin users, verify they're changing their own password
    if claims.role != "admin" {
        match authentication::get_user_email(pool, &claims.sub).await {
            Ok(Some(user_email)) => {
                if user_email != new_password.email {
                    return HttpResponse::Forbidden().json(json!({
                        "success": false,
                        "message": "You can only change your own password"
                    }));
                }
            }
            Ok(None) => {
                return HttpResponse::NotFound().json(json!({
                    "success": false,
                    "message": "User not found"
                }));
            }
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": "Failed to verify user"
                }));
            }
        }

        // Verify current password for non-admin users
        match authentication::verify_current_password(
            pool,
            &new_password.email,
            &claims.role,
            &new_password.current_password
        ).await {
            Ok(true) => (),  // Password verified, continue
            Ok(false) => {
                return HttpResponse::Unauthorized().json(json!({
                    "success": false,
                    "message": "Current password is incorrect"
                }));
            }
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "message": "Failed to verify current password"
                }));
            }
        }
    }

    // Hash the new password
    let hashed_password = match hash(&new_password.new_password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Password generation failed"
            }));
        }
    };

    // Update the password in the database
    match authentication::update_password(pool, &claims.role, &new_password.email, &hashed_password).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "success": true,
            "message": "Password updated successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "success": false,
            "message": "Failed to update password"
        })),
    }
}

#[post("/register-with-admin")]
pub async fn admin_create_user(
    data: web::Data<crate::AppState>,
    claims: web::ReqData<Claims>,
    register_req: web::Json<RegisterRequest>,
) -> HttpResponse {
    let pool = &data.db;
    if claims.role != "admin" {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "message": "Only admin can create user"
        }));
    }
    // Hash the password
    let hashed_password = match hash(&register_req.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => {
            return HttpResponse::InternalServerError().json(LoginResponse {
                success: false,
                message: "Password hashing failed".to_string(),
                data: None,
                user_data: None,
            });
        }
    };

    // Attempt to create the user
    match authentication::create_user(
        pool,
        &register_req.email,
        &hashed_password,
        &register_req.name,
        &register_req.role,
    )
    .await
    {
        Ok(_) => (),
        Err(_) => {
            return HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Failed to create user"
            }));
        }
    }

    if register_req.role == "doctor" {
        match doctor::update_doctor_speciality(
            pool,
            register_req.email.clone(),
            &register_req.speciality_id.unwrap(),
        )
        .await
        {
            Ok(_) => HttpResponse::Ok().json(json!({
                "success": true,
                "message": "Doctor registered successfully"
            })),
            Err(_) => {
                return HttpResponse::InternalServerError().json(json!({
                "success": false,
                    "message": "Failed to update doctor speciality"
                }));
            }
        }
    } else {
        HttpResponse::Ok().json(json!({
            "success": true,
            "message": "User registered successfully"
        }))
    }
}

