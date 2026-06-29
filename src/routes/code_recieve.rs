// recieve code to run

use crate::docker_worker::runner::init_run;
use crate::middleware::middleware::Claims;
use actix_web::{HttpMessage, HttpResponse, Responder, post, web};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct CodeBody {
    code_content: String,
    file_name: String,
    language: String,
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
        .collect()
}

#[post("/code/upload")]
pub async fn executeCode(
    req: actix_web::HttpRequest,
    content: web::Json<CodeBody>,
) -> impl Responder {
    let claims = match req.extensions().get::<Claims>() {
        Some(c) => c,
        None => return HttpResponse::Unauthorized().json("missing auth"),
    };

    let username = match req.extensions().get::<Claims>() {
        Some(claims) => claims.username.clone(),
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let base_dir = std::env::var("STORAGE_DIR").unwrap_or_else(|_| "./store".to_string());

    let user_dir = PathBuf::from(base_dir).join(username);

    let safe_name = sanitize_filename(&content.file_name);
    if safe_name.is_empty() {
        return HttpResponse::BadRequest().json("invalid filename");
    }
    let file_path = user_dir.join(safe_name);

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

    init_run(
        file_path.to_string_lossy().into_owned(),
        content.language.clone(),
    );

    HttpResponse::Ok().json("api hit")
}
