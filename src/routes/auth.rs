use actix_web::{HttpResponse, Responder, post, web};
use sqlx::PgPool;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use std::env;

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use chrono;

#[derive(Deserialize)]
struct AuthData {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}

#[derive(Clone, sqlx::FromRow)]
struct User {
    username: String,
    password_hash: String,
}

#[derive(Serialize)]
struct AuthReponseSuccess {
    token: String,
}

#[derive(Serialize)]
struct AuthResponseFailure {
    fail_reason: String,
}
fn generate_token(username: &str) -> String {
    let claims = Claims {
        username: username.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let secret = env::var("JWT_SECRET").unwrap();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
#[post("/api/register")]
async fn register(pool: web::Data<PgPool>, req_body: web::Json<AuthData>) -> impl Responder {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(req_body.password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash)
         VALUES ($1, $2)",
    )
    .bind(&req_body.username)
    .bind(&password_hash)
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(_) => {
            let token = generate_token(&req_body.username);
            HttpResponse::Ok().json(AuthReponseSuccess { token })
        }
        Err(e) => HttpResponse::InternalServerError().json(AuthResponseFailure {
            fail_reason: e.to_string(),
        }),
    }
}
//eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6InJpc2hhYmgiLCJleHAiOjIwMDAwMDAwMDB9.IfonnzBnE-1zaV3SMDkF5ukRyXj4wdAsKfq0I1NmAFk

/*
curl -X POST http://localhost:8080/code/upload \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6InJpc2hhYmgiLCJleHAiOjIwMDAwMDAwMDB9.IfonnzBnE-1zaV3SMDkF5ukRyXj4wdAsKfq0I1NmAFk" \
  -d '{
    "code_content": "print(\"hello world\")",
    "language": "python",
    "username": "rishabh",
    "file_name":"python1"
  }'
*/
#[post("/api/login")]
async fn login(pool: web::Data<PgPool>, req_body: web::Json<AuthData>) -> impl Responder {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&req_body.username)
        .fetch_one(pool.get_ref())
        .await;

    match user {
        Ok(user) => {
            let parsed_hash = match PasswordHash::new(&user.password_hash) {
                Ok(h) => h,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(AuthResponseFailure {
                        fail_reason: "invalid password hash".to_string(),
                    })
                }
            };

            if Argon2::default()
                .verify_password(req_body.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                let token = generate_token(&user.username);
                HttpResponse::Ok().json(AuthReponseSuccess { token })
            } else {
                HttpResponse::Unauthorized().json(AuthResponseFailure {
                    fail_reason: "invalid credentials".to_string(),
                })
            }
        }

        Err(_) => HttpResponse::Unauthorized().json(AuthResponseFailure {
            fail_reason: "invalid credentials".to_string(),
        }),
    }
}
