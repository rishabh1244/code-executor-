use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use actix_web_httpauth::middleware::HttpAuthentication;

use dotenvy::dotenv;
use std::env;

mod db;
mod middleware;
mod routes;

use db::connect::connect_db;
use middleware::middleware::validator;
use routes::auth::{login, register};
use routes::code_recieve::executeCode;

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
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(web::Data::new(pgPool.clone()))
            .service(hello)
            .service(register)
            .service(login)
            .service(web::scope("/code").wrap(auth).service(executeCode))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
