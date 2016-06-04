use std::collections::HashMap;
use entity::*;
use world::World;
use command::Command;
use std::sync::mpsc::Sender;
use std::cell::RefCell;

pub struct Room {
    pub name: String,
    description: String,
    pub entities: HashMap<usize, RefCell<Entity>>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            name: "New room".to_string(),
            description: "You see an empty room.".to_string(),
            entities: HashMap::new()
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, RefCell::new(entity));
    }

    pub fn get_entity(&self, id: usize) -> Option<&RefCell<Entity>> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: usize) -> Option<&mut RefCell<Entity>> {
        self.entities.get_mut(&id)
    }

    pub fn remove_entity(&mut self, id: usize) -> Option<RefCell<Entity>> {
        self.entities.remove(&id)
    }

    pub fn tick(&self, world: &World, sender: Sender<Command>) {
        for (_, mut entity) in self.entities.iter() {
            
            let entity = &mut *entity.borrow_mut();

            match entity.value {
                EntityValue::Creature(ref mut creature) => {
                    creature.tick(world, sender.clone());
                }
            }
        };
    }
}
