use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use dotenvy::dotenv;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: usize,
}

async fn gen_token() {
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default()
        .verify_password(req_body.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return HttpResponse::Unauthorized().finish();
    }
    let claims = Claims {
        username: user.username,
        exp: 2000000000,
    };

    let secret = env::var("JWT_SECRET").unwrap();

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
}
