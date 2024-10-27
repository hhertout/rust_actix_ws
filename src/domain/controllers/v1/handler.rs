use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError}, get, http::StatusCode, web::{self, Data}, Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use serde_json::json;

use crate::{
    app::ws::{socket_worker::SocketWorker, socket_server::ChatServer},
    AppState,
};

#[get("/ws")]
pub async fn chat(
    req: HttpRequest,
    stream: web::Payload,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let uid = {
        let mut srv = state.ws_server.lock().unwrap();
        srv.gen_uid()
    };


    log::debug!(
        "New websocket connection handled for user={:?}, room_id={:?}...",
        "todo",
        "todo"
    );

    ws::start(SocketWorker::new(uid, state.ws_server.clone()), &req, stream)
}
