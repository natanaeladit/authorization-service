use actix_web::{get, App, HttpResponse, HttpServer, Responder};

extern crate dotenv;
extern crate env_logger;

use std::{env};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");
    let app_url = format!("{}:{}", &app_host, &app_port);

    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(&app_url)?
    .run()
    .await
}