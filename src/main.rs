mod application;

use crate::application::controllers::health::health;
use actix_web::{web, App, HttpServer};
use application::controllers::gen_v1;
use domain::ws::socket_server::ChatServer;
use std::sync::{Arc, Mutex};

pub(crate) mod domain;
mod infra;

#[derive(Clone)]
pub struct AppState {
    ws_server: Arc<Mutex<ChatServer>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = Arc::new(Mutex::new(ChatServer::new()));
    let state = AppState {
        ws_server: server.clone(),
    };

    let ipv4 = "0.0.0.0";
    let port = std::env::var("PORT").unwrap_or_else(|_| String::from("4000"));

    log::info!("🚀 App will start on port {}:{}", ipv4, port);
    // For bind ssl certificates : https://actix.rs/docs/server#tls--https
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(gen_v1())
    })
    .bind((ipv4, port.parse::<u16>().expect("Port cannot be parsed")))?
    .shutdown_timeout(10)
    .run()
    .await
}
