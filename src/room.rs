use std::collections::HashMap;
use entity::*;
use creature::Creature;
use world::World;
use command::Command;
use std::sync::mpsc::Sender;
use item::ItemSpawnDefinition;

pub struct Room {
    id: String,
    pub name: String,
    pub description: String,
    exits: Vec<(String, String)>, // these are (exit name, room id)
    pub creatures: HashMap<usize, Creature>,
    itemSpawnDefinitions: Vec<ItemSpawnDefinition>,
}

impl Room {

    pub fn new(id: String) -> Room {
        Room {
            id: id.clone(),
            name: id,
            description: "You see an empty room.".to_string(),
            exits: Vec::new(),
            creatures: HashMap::new(),
            itemSpawnDefinitions: Vec::new(),
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

    pub fn add_creature(&mut self, entity: Creature) {
        self.creatures.insert(entity.get_id(), entity);
    }

    pub fn get_creature(&self, id: usize) -> Option<&Creature> {
        self.creatures.get(&id)
    }

    pub fn get_creature_mut(&mut self, id: usize) -> Option<&mut Creature> {
        self.creatures.get_mut(&id)
    }

    pub fn remove_creature(&mut self, id: usize) -> Option<Creature> {
        self.creatures.remove(&id)
    }

    pub fn tick(&self, world: &World, sender: Sender<Command>) {
        for (_, creature) in self.creatures.iter() {
            if let Some(creature) = creature.as_tick() {
                creature.tick(self, world, sender.clone());
            }
        };
    }
}
