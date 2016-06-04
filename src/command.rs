use entity::*;

pub enum Command {
    Add {
        entity: Entity,
        location: String,
    },
    Remove {
    	id: usize,
    	location: String,
    },
    Move {
        from: String,
        id: usize,
        to: String
    }
}
