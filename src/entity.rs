use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::mpsc::Sender;
use command::Command;
use room::Room;
use world::World;
use player::Player;
use item::Thing;
use creature::Creature;

static ENTITY_ID_GENERATOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub trait AsEntity {
    fn as_entity(self) -> Entity;
}

pub trait Item {
    fn get_item_type(&self) -> &str;
}

pub trait Stackable: Item {
    fn get_stack_count(&self) -> u64;
}

pub trait Describable {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

pub trait Id {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

pub trait Tick {
    fn tick(&self, room: &Room, world: &World, sender: Sender<Command>);
}

pub fn generate_id() -> usize {
    ENTITY_ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

pub enum Entity {
    Player(Player),
    Thing(Thing),
}

impl Entity {
    pub fn get_id(&self) -> usize {
        match self {
            &Entity::Player(ref player) => player.get_id(),
            &Entity::Thing(ref thing) => thing.get_id()
        }
    }

    pub fn as_tickable(&self) -> Option<&Tick> {
        match self {
            &Entity::Player(ref player) => Some(player),
            _ => None,
        }
    }

    pub fn as_creature(&self) -> Option<&Creature> {
        match self {
            &Entity::Player(ref player) => Some(player),
            _ => None
        }
    }

    pub fn as_item(&self) -> Option<&Item> {
        match self {
            &Entity::Thing(ref thing) => Some(thing),
            _ => None
        }
    }

    pub fn as_describable(&self) -> Option<&Describable> {
        match self {
            &Entity::Player(ref player) => Some(player),
            &Entity::Thing(ref thing) => Some(thing)
        }
    }
}
