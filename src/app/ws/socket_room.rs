use crate::infra::repository::room_repository::RoomRepository;

pub struct SocketRoom {
    repository: RoomRepository,
}

impl SocketRoom {
    pub fn new(repository: RoomRepository) -> Self {
        SocketRoom {
            repository
        }
    }

    pub fn get_room(&mut self, id: String) {
        let room = self.repository.find_one_by(id);
    }
    pub fn create_room(&self) {}
    pub fn delete_room(&self) {}
}
