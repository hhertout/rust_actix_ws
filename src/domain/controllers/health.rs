use actix_web::{get, HttpResponse, Responder};

#[get("/heath")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm alive")
}