use actix::ActorContext;
use actix::AsyncContext;
use actix::Handler;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::socket_server::ChatMessage;
use super::socket_server::ChatServer;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Clone)]
pub struct SocketWorker {
    instance_id: Uuid,
    server: Arc<Mutex<ChatServer>>,
    hb: Instant,
}

impl SocketWorker {
    pub fn new(instance_id: Uuid, server: Arc<Mutex<ChatServer>>) -> Self {
        SocketWorker {
            instance_id,
            server,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                log::error!("Timeout for client exceed, disconnecting...");
                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for SocketWorker {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let mut server = self.server.lock().expect("Failed to lock the server mutex");
        server.add_session(self.instance_id, ctx.address().recipient());
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        let mut server = self.server.lock().expect("Failed to lock the server mutex");
        server.remove_session(self.instance_id);
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketWorker {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();

                ctx.pong(&msg)
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let server = self.server.lock().expect("Failed to lock the server mutex");
                log::debug!("Received message :  {}", text);
                server.send_message(&text, self.instance_id);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                if reason.is_none() {
                    log::debug!("Socket disconnected for unknow reason");
                } else {
                    log::debug!("Socket close : {:?}", reason);
                }

                ctx.close(reason);
                ctx.stop();
            }
            Err(err) => {
                log::error!("Message error : {}", err);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<ChatMessage> for SocketWorker {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
