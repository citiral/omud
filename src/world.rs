use room;
use std::collections::HashMap;

pub struct World<'a> {
    rooms: HashMap<&'a str, room::Room<'a>>,
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            rooms: HashMap::new()
        }
    }

    pub fn getRoom(&self, identifier: &'a str) -> Option<&'a room::Room> {
        self.rooms.get(identifier)
    }

    pub fn addRoom(&mut self, identifier: &'a str, room: room::Room<'a>) {
        self.rooms.insert(identifier, room);
    }
}
