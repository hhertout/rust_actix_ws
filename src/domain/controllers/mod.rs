use actix_web::{web, Scope};
use v1::handler::chat;

pub(crate) mod health;
pub(crate) mod v1;

pub fn gen_v1() -> Scope {
    web::scope("/api/v1").service(chat)
}
