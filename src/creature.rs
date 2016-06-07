use player::Player;
use entity::{Id, Tick, Init};

pub enum Creature {
    Player(Player),
}

impl Creature {
    pub fn get_id(&self) -> usize {
        match self {
            &Creature::Player(ref player) => player.get_id()
        }
    }

    pub fn as_id(&self) -> &Id {
        match self {
            &Creature::Player(ref player) => player
        }
    }

    pub fn as_tick(&self) -> Option<&Tick> {
        match self {
            &Creature::Player(ref player) => Some(player)
        }
    }

    pub fn as_init(&self) -> Option<&Init> {
        match self {
            &Creature::Player(ref player) => None
        }
    }
}