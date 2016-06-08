use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::sync::mpsc::Sender;
use std::iter::Iterator;

use command::Command;
use room::Room;
use world::World;
use item::Item;

static ENTITY_ID_GENERATOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn generate_id() -> usize {
    ENTITY_ID_GENERATOR.fetch_add(1, Ordering::Relaxed)
}

pub trait Describable {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

pub trait Id {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

pub trait Init {
	fn init(&self, room: &Room, world: &World, sender: Sender<Command>);
}

pub trait Tick {
    fn tick(&self, room: &Room, world: &World, sender: Sender<Command>);
}

pub trait Container {
	fn has_item(&self, id: usize) -> bool;
	fn add_item(&mut self, item: Item);
	fn get_item(&self, id: usize) -> Option<&Item>;
	fn get_item_mut(&mut self, id: usize) -> Option<&mut Item>;
	fn remove_item(&mut self, id: usize) -> Option<Item>;

	fn item_count(&self) -> usize;
}