use crate::domain::value_object::room::Room;
use redis::{Commands, Connection, RedisError};

pub struct RoomRepository {
    pool: Connection,
}

impl RoomRepository {
    pub fn new(pool: Connection) -> Self {
        RoomRepository { pool }
    }

    pub fn create_room(&mut self, room: &Room) -> Result<(), RedisError> {
        let json = serde_json::to_string(&room.users).unwrap();

        log::debug!("Inserting users {:?} in room_id={:?}", room.id, json);
        let id = room.id.to_string();
        redis::cmd("SET").arg(id).arg(json).query(&mut self.pool)
    }

    pub fn find_one_by(&mut self, id: String) -> Result<String, RedisError> {
        self.pool.get(&id)
    }

    pub fn delete_room(&mut self, id: String) -> Result<(), RedisError> {
        self.pool.del(&id)
    }

    pub fn get_room_count(&mut self) -> Result<usize, RedisError> {
        redis::cmd("DBSIZE").query(&mut self.pool)
    }
}
