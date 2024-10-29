mod app;

use std::sync::{Arc, Mutex};
use crate::domain::controllers::health::health;
use actix_web::{web, App, HttpServer};
use app::ws::socket_server::ChatServer;
use domain::controllers::gen_v1;

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

    log::info!("🚀 App will start on port {}", 8080);
    // For bind ssl certificates : https://actix.rs/docs/server#tls--https
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(gen_v1())
    })
        .bind(("127.0.0.1", 8080))?
        .shutdown_timeout(10)
        .run()
        .await
}
