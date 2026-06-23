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
    println!("username: {}", req_body.username);
    println!("password: {}", req_body.password);
    HttpResponse::Ok().body("registerd")
}
