use creature::*;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

static ENTITY_ID_GENERATOR: AtomicUsize = ATOMIC_USIZE_INIT;

pub enum EntityValue {
    Creature(Box<Creature + Send>)
}

pub struct Entity {
    pub id: usize,
    pub value: EntityValue,
}

impl Entity {
    pub fn new(value: EntityValue) -> Entity {
        Entity {
            id: ENTITY_ID_GENERATOR.fetch_add(1, Ordering::Relaxed),
            value: value
        }
    }
}
