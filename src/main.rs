use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};

use dotenvy::dotenv;
use std::env;

mod db;
mod routes;

use db::connect::connect_db;
use routes::auth::{login, register};

#[post("/hello")]
async fn hello(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

    let pgPool = connect_db(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to postgres!");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pgPool.clone()))
            .service(hello)
            .service(register)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
