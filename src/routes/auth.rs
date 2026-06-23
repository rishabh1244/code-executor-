use std::fmt::write;

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct RegisterData {
    username: String,
    password: String,
}
#[post("/api/register")]
async fn register(pool: web::Data<PgPool>, req_body: web::Json<RegisterData>) -> impl Responder {
    let result = sqlx::query(
        "INSERT INTO users (username, password_hash)
         VALUES ($1, $2)",
    )
    .bind(&req_body.username)
    .bind(&req_body.password)
    .execute(pool.get_ref())
    .await;
    println!("username  : {}", req_body.password);

    println!("username  : {}", req_body.username);
    match result {
        Ok(_) => HttpResponse::Ok().body("registered"),
        Err(e) => {
            println!("DB Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
