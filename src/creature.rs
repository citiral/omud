use entity::{Id, Tick, Describable};

pub trait Creature: Tick + Id + Describable {
    fn get_health(&self) -> f32;
}
