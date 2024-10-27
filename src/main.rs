mod app;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::domain::controllers::health::health;

pub(crate) mod domain;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // For bind ssl certificates : https://actix.rs/docs/server#tls--https
    HttpServer::new(|| {
        App::new().service(health)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
