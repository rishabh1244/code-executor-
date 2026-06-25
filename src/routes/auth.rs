use std::fmt::write;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use sqlx::PgPool;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use dotenvy::dotenv;
use std::env;

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

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
        exp: 2_000_000_000,
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

#[post("/api/login")]
async fn login(pool: web::Data<PgPool>, req_body: web::Json<AuthData>) -> impl Responder {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&req_body.username)
        .fetch_one(pool.get_ref())
        .await;

    match user {
        Ok(user) => {
            let token = generate_token(&user.username);
            HttpResponse::Ok().json(AuthReponseSuccess { token })
        }

        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}
