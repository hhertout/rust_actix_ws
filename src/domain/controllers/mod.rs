use actix_web::web;

pub(crate) mod health;
pub(crate) mod v1;

pub fn gen_v1() {
    web::scope("/api/v1");
}