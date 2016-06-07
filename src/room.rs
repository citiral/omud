use std::collections::HashMap;
use entity::*;
use world::World;
use command::Command;
use std::sync::mpsc::Sender;

pub struct Room {
    id: String,
    pub name: String,
    pub description: String,
    exits: Vec<(String, String)>, // these are (exit name, room id)
    pub entities: HashMap<usize, Entity>,
}

impl Room {

    pub fn new(id: String) -> Room {
        Room {
            id: id.clone(),
            name: id,
            description: "You see an empty room.".to_string(),
            exits: Vec::new(),
            entities: HashMap::new()
        }
    }

    pub fn get_exit(&self, exit: &str) -> Option<String> {
        for &(ref name, ref id) in &self.exits {
            if name == exit {
                return Some(id.clone())
            }
        }
        None
    }

    pub fn add_exit(&mut self, name: String, id: String) {
        self.exits.push((name, id));
    }

    pub fn connect_to_room(&mut self, room: &mut Room, here: String, there: String) {
        self.exits.push((here, room.id.clone()));
        room.exits.push((there, self.id.clone()));
    }

    pub fn connect_to_room_one_way(&mut self, room: &mut Room, here: String) {
        self.exits.push((here, room.id.clone()));
    }

    pub fn exits(&self) -> &Vec<(String, String)> {
        &self.exits
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.get_id(), entity);
    }

    pub fn get_entity(&self, id: usize) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: usize) -> Option<Entity> {
        self.entities.remove(&id)
    }

    pub fn tick(&self, world: &World, sender: Sender<Command>) {
        for (_, entity) in self.entities.iter() {
            if let Some(entity) = entity.as_tickable() {
                entity.tick(self, world, sender.clone());
            }
        };
    }
}
