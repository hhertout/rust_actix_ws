use actix_web::{web, Scope};
use v1::websocket::chat;
use crate::domain::controllers::v1::room::{create_room, delete_room, get_room_number};

pub(crate) mod health;
pub(crate) mod v1;

pub fn gen_v1() -> Scope {
    web::scope("/api/v1")
        .service(chat)
        .service(get_room_number)
        .service(create_room)
        .service(delete_room)
}
