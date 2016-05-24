use std::collections::LinkedList;
use entity;

pub struct Room<'a> {
    name: &'a str,
    description: &'a str,
    entities: LinkedList<entity::Entity>,
}

impl<'a> Room<'a> {
    pub fn new() -> Room<'a> {
        Room {
            name: "New room",
            description: "You see an empty room.",
            entities: LinkedList::new()
        }
    }
}
