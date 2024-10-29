use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: usize,
    pub users: Vec<String>,
}
