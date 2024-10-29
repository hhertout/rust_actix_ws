use actix_web::{delete, get, post, HttpResponse, Responder};
use actix_web::http::StatusCode;

#[get("/room/room-number")]
pub async fn get_room_number() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
}

#[post("/room")]
pub async fn create_room() -> impl Responder {
    HttpResponse::build(StatusCode::CREATED)
}

#[delete("/room")]
pub async fn delete_room() -> impl Responder {
    HttpResponse::build(StatusCode::NO_CONTENT)
}