use entity::*;

pub enum Command {
    Add {
        entity: Entity,
        location: String,
    },
    Remove {
    	id: i32,
    	location: String,
    },
    Move {
        from: String,
        id: i32,
        to: String
    }
}
