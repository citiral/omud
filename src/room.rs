use std::collections::HashMap;
use entity::*;
use world::World;
use command::Command;
use std::sync::mpsc::Sender;
use std::cell::{RefCell, Ref, RefMut};
use std::iter;

pub struct Room {
    id: String,
    name: String,
    description: String,
    pub entities: HashMap<usize, RefCell<Entity>>,
}

impl Room {
    pub fn new(id: String) -> Room {
        Room {
            id: id,
            name: "New room".to_string(),
            description: "You see an empty room.".to_string(),
            entities: HashMap::new()
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.get_id(), RefCell::new(entity));
    }

    pub fn get_entity(&self, id: usize) -> Option<Ref<Entity>> {
        if let Some(entity) = self.entities.get(&id) {
            Some(entity.borrow())
        } else {
            None
        }
    }

    pub fn get_entity_mut(&mut self, id: usize) -> Option<RefMut<Entity>> {
        if let Some(entity) = self.entities.get_mut(&id) {
            Some(entity.borrow_mut())
        } else {
            None
        }
    }

    pub fn remove_entity(&mut self, id: usize) -> Option<Entity> {
        if let Some(entity) = self.entities.remove(&id) {
            Some(entity.into_inner())
        } else {
            None
        }
    }

    pub fn tick(&self, world: &World, sender: Sender<Command>) {
        for (_, entity) in self.entities.iter() {

            let mut entity = entity.borrow_mut();

            match *entity {
                Entity::Creature(ref mut creature) => {
                    creature.tick(self, world, sender.clone());
                }
            }
        };
    }
}
