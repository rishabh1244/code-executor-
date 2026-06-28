// recieve code to run

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use std::fs;
use std::path::PathBuf;
#[derive(Deserialize)]
struct codeBody {
    code_content: String,
    file_name: String,
    language: String,
    username: String,
}

#[post("/code/upload")]
pub async fn executeCode(pool: web::Data<PgPool>, content: web::Json<codeBody>) -> impl Responder {
    // store it in a folder,
    // folder structure /username/{fileName}

    let base_dir = std::env::var("STORAGE_DIR").unwrap_or_else(|_| "./store".to_string());
    let user_dir = PathBuf::from(base_dir).join(&content.username);
    let file_path = user_dir.join(&content.file_name);
    if let Err(e) = fs::create_dir_all(&user_dir) {
        eprintln!("failed to create dir {:?}: {e}", user_dir);
        return HttpResponse::InternalServerError().json("failed to create user directory");
    }

    if let Err(e) = fs::write(&file_path, &content.code_content) {
        eprintln!("failed to write file {:?}: {e}", file_path);
        return HttpResponse::InternalServerError().json("failed to write file");
    }

    match fs::canonicalize(&file_path) {
        Ok(abs_path) => println!("absolute path: {:?}", abs_path),
        Err(e) => eprintln!("canonicalize failed: {e}"),
    }

    HttpResponse::Ok().json("api hit")
}
