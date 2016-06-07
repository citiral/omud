use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::mpsc::Sender;
use command::Command;
use room::Room;
use world::World;
use player::Player;
use creature::Creature;

static ENTITY_ID_GENERATOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn generate_id() -> usize {
    ENTITY_ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
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

