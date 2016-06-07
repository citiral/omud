use room::*;
use event::*;
use item::ItemDefinition;

use std::collections::HashMap;
use command::Command;
use std::sync::mpsc::Sender;

pub struct World {
    rooms: HashMap<String, Room>,
    items: HashMap<String, ItemDefinition>,
    events: HashMap<String, Event>,
}

impl World {
    pub fn new() -> World {
        World {
            rooms: HashMap::new(),
            events: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub fn get_event(&self, identifier: &str) -> Option<&Event> {
        self.events.get(identifier)
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.insert(event.get_id().to_string(), event);
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

    pub fn add_item_definition(&mut self, item: ItemDefinition) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn get_item_definition(&self, id: &str) -> Option<&ItemDefinition> {
        self.items.get(id)
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
