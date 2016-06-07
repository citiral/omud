use item::Item;
use creature::Creature;
use world::World;

pub enum Command {
    AddCreature {
        creature: Creature,
        location: String,
    },
    AddItem {
    	item: Item,
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

impl Command {
    pub fn execute(self, world: &mut World) -> Result<(), &str> {
        match self {
            Command::AddCreature{creature, location} => {
                if let Some(room) = world.get_room_mut(&location) {
                    room.add_creature(creature);
                    Ok(())
                } else {
                    Err("Add: No such room found")
                }
            },
            Command::AddItem{item, location} => {
                if let Some(room) = world.get_room_mut(&location) {
                    room.add_item(item);
                    Ok(())
                } else {
                    Err("Add: No such room found")
                }
            },
            Command::Remove{id, location} => {
                if let Some(room) = world.get_room_mut(&location) {
                    room.remove_creature(id);
                    Ok(())
                } else {
                    Err("Remove: no such room found")
                }
            },
            Command::Move{id, from, to} => {
                let entity = match world.get_room_mut(&from) {
                    Some(room) => room.remove_creature(id),
                    None => None
                };

                if let Some(e) = entity {
                    if let Some(room) = world.get_room_mut(&to) {
                        room.add_creature(e);
                        Ok(())
                    } else {
                        Err("Move: to room not found")
                    }
                } else {
                    Err("Move: entity not found or from room not found")
                }
            }
        }
    }
}
