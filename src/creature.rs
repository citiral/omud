use entity::{Id, Tick, Describable};
use player::Player;

pub trait Creature: Tick + Id + Describable {
    fn get_health(&self) -> f32;
}
