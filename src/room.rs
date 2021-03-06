use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::iter::Iterator;

use entity::*;
use creature::Creature;
use world::World;
use command::Command;
use item::{ItemSpawn, Item};
use WORLD_DATA;

pub struct Room {
    id: String,
    pub name: String,
    pub description: String,
    exits: Vec<(String, String)>, // these are (exit name, room id)
    pub creatures: HashMap<usize, Creature>,
    pub items: HashMap<usize, Item>,
    item_spawns: Vec<ItemSpawn>,
}

impl Room {

    pub fn new(id: String) -> Room {
        Room {
            id: id.clone(),
            name: id,
            description: "You see an empty room.".to_string(),
            exits: Vec::new(),
            creatures: HashMap::new(),
            items: HashMap::new(),
            item_spawns: Vec::new(),
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

    pub fn add_item_spawn(&mut self, spawn: ItemSpawn) {
        self.item_spawns.push(spawn);
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.get_id(), item);
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

    pub fn init(&self, world: &World, sender: Sender<Command>) {
        // spawn the initial items
        for spawn in &self.item_spawns {
            if let Some(definition) = WORLD_DATA.items.get(&spawn.get_id()) {
                if definition.stackable {
                    let _ = sender.send(Command::AddItem {
                        item: definition.spawn(spawn.get_count()),
                        location: self.id.clone(),
                    });
                } else {
                    for _ in 0..spawn.get_count() {
                        let _ = sender.send(Command::AddItem {
                            item: definition.spawn(1),
                            location: self.id.clone(),
                        });
                    }
                }
            }
        }

        for (_, creature) in self.creatures.iter() {
            if let Some(creature) = creature.as_init() {
                creature.init(self, world, sender.clone());
            }
        };
    }

    pub fn tick(&self, world: &World, sender: Sender<Command>) {
        for (_, creature) in self.creatures.iter() {
            if let Some(creature) = creature.as_tick() {
                creature.tick(self, world, sender.clone());
            }
        };
    }
}
