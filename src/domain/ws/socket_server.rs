use std::collections::HashMap;

use actix::{Message, Recipient};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Clone)]
pub struct ChatServer {
    pub uid: Uuid,
    pub sessions: HashMap<Uuid, Recipient<ChatMessage>>,
}

impl ChatServer {
    pub fn new() -> Self {
        log::debug!("Creating a new websocket chat server");
        ChatServer {
            uid: Uuid::new_v4(),
            sessions: HashMap::new(),
        }
    }

    pub fn gen_uid(&mut self) -> Uuid {
        self.uid = Uuid::new_v4();

        self.uid
    }

    pub fn add_session(&mut self, uid: Uuid, addr: Recipient<ChatMessage>) {
        log::debug!("New session added: {}", uid);
        self.sessions.insert(uid, addr);
    }

    pub fn remove_session(&mut self, uid: Uuid) {
        self.sessions.remove(&uid);
    }

    pub fn session_len(&self) -> usize {
        self.sessions.len()
    }

    pub fn send_message(&self, message: &str, skip_id: Uuid) {
        log::debug!("user_uid={:?}", self.uid);
        for (id, recipient) in &self.sessions {
            if *id != skip_id {
                log::debug!("Message sent to id={}", id);
                recipient.do_send(ChatMessage(message.to_owned()));
            }
        }
    }
}
