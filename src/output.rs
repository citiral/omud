use std::io::Write;

use room::Room;
use world::World;
use item::Item;
use entity::{Id, Describable, Living};
use player::Player;
use creature::Creature;

fn write<O: Write>(output: &mut O, line: &str) {
    output.write(line.as_bytes()).unwrap();
}

fn writeln<O: Write>(output: &mut O, line: &str) {
    output.write(line.as_bytes()).unwrap();
    output.write(b"\r\n").unwrap();
}

pub fn echo<O: Write>(output: &mut O, command: &Vec<&str>) {
    if command.len() > 1 {
        writeln(output, command[1]);
    } else {
        writeln(output, command[0]);
    }
}

pub fn unknown_command<O: Write>(output: &mut O, command: &str) {
    writeln(output, &format!("Unknown command: {}", command));
}

pub fn status<O: Write, L: Living + Describable>(output: &mut O, creature: &L) {
    write(output, "Status of ");
    write(output, &creature.get_name());
    writeln(output, "");
    write(output, "health: ");
    write(output, &format!("{}", creature.get_health()));
    writeln(output, "");
}

pub fn look<O: Write>(output: &mut O, player: &Player, room: &Room) {
    writeln(output, &room.name);
    writeln(output, "");
    writeln(output, &room.description);
    writeln(output, "");

    write(output, "Creatures here: ");
    for creature in room.creatures.values() {
        if let Some(describable) = creature.as_describable() {
            if creature.get_id() == player.get_id() {
                write(output, "you ");
            } else {
                write(output, &describable.get_name());
                write(output, " ");
            }
        }
    }
    writeln(output, "");

    if !room.items.is_empty() {
        write(output, "Items here: ");
        for item in room.items.values() {
            write(output, &item.get_name());
        }
        writeln(output, "");
    }

    writeln(output, "");

    write(output, "Exits: ");
    for &(ref name, _) in room.exits() {
        write(output, &name);
        write(output, " ");
    }

    writeln(output, "");
}
