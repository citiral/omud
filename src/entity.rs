use creature::*;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::mpsc::Sender;
use command::Command;
use room::Room;
use world::World;

static ENTITY_ID_GENERATOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub trait Describable {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

pub trait Id {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

pub trait Tick {
    fn tick(&mut self, room: &Room, world: &World, sender: Sender<Command>);
}

pub fn generate_id() -> usize {
    ENTITY_ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

pub enum Entity {
    Creature(Box<Creature + Send>)
}

impl Entity {
    pub fn get_id(&self) -> usize {
        match self {
            &Entity::Creature(ref creature) => creature.get_id()
        }
    }
}
