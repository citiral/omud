use room::*;
use item::ItemDefinition;

use std::collections::HashMap;
use command::Command;
use std::sync::mpsc::Sender;
use datastore::DataStore;

pub struct WorldData {
    pub items: DataStore<ItemDefinition>
}

impl WorldData {
    pub fn new() -> WorldData {
        WorldData {
            items: DataStore::new()
        }
    }
}

pub struct World {
    rooms: HashMap<String, Room>
}

impl World {
    pub fn new() -> World {
        World {
            rooms: HashMap::new(),
        }
    }

    pub fn get_room(&self, identifier: &str) -> Option<&Room> {
        self.rooms.get(identifier)
    }

    pub fn get_room_mut(&mut self, identifier: &str) -> Option<&mut Room> {
        self.rooms.get_mut(identifier)
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.get_id().to_string(), room);
    }

    pub fn tick(&self, sender: Sender<Command>) {
        for (_, room) in self.rooms.iter() {
            room.tick(self, sender.clone());
        }
    }

    pub fn init(&self, sender: Sender<Command>) {
        for (_, room) in self.rooms.iter() {
            room.init(self, sender.clone());
        }
    }
}
