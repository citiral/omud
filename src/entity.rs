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

pub trait Living {
    fn get_health(&self) -> f32;
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

pub trait Container<'a> {
    type Iter: Iterator;

	fn has_item(&'a self, id: usize) -> bool;
	fn add_item(&'a mut self, item: Item);
	fn get_item(&'a self, id: usize) -> Option<&'a Item>;
	fn get_item_mut(&'a mut self, id: usize) -> Option<&'a mut Item>;
	fn remove_item(&'a mut self, id: usize) -> Option<Item>;

	fn item_count(&'a self) -> usize;

    fn inventory_iter(&'a self) -> Self::Iter;
}
