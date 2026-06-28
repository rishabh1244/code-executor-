// recieve code to run

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
struct codeBody {
    code_content: String,
    language: String,
    username: String,
}

#[post("/code/upload")]
pub async fn executeCode(pool: web::Data<PgPool>, content: web::Json<codeBody>) -> impl Responder {
    print!("{}", content.code_content);
    HttpResponse::Ok().json("api hit")
}
