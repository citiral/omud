use entity::{Id, Tick, Describable};
use player::Player;
use room::Room;
use world::World;
use command::Command;
use std::sync::mpsc::Sender;

pub trait Creature: Tick + Id + Describable {
    fn get_health(&self) -> f32;
}
