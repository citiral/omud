use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use world::World;
use std::sync::mpsc::Sender;
use command::Command;
use room::Room;
use entity::{self, Entity, Id, Tick, Describable};


pub trait Creature: Tick + Id + Describable {
    fn get_health(&self) -> f32;
}

pub struct Player {
    id: usize,
    name: String,
    health: f32,
    connected: bool,
    stream: TcpStream,
}

impl Id for Player {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Creature for Player {
    fn get_health(&self) -> f32 {
        self.health
    }
}

impl Tick for Player {
    fn tick(&mut self, room: &Room, world: &World, sender: Sender<Command>) {
        let line = self.read_line().trim().to_string();

        if line != "" {
            self.handle_command(room, world, sender, &line);
        }
    }
}

impl Describable for Player {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        "player ".to_string() + &self.name
    }
}

impl Player {
    pub fn new(stream: TcpStream) -> Player {
        Player {
            id: entity::generate_id(),
            name: "Bob".to_string(),
            health: 100.0,
            connected: false,
            stream: stream,
        }
    }

    fn handle_command(&mut self, room: &Room, world: &World, sender: Sender<Command>, command: &str) {
        let parts: Vec<&str> = command.splitn(2, ' ').collect();

        match parts[0] {
            "echo" => {
                if parts.len() > 1 {
                    self.writeln(parts[1]);
                } else {
                    self.writeln(parts[0]);
                }
            },
            "exit" | "quit" => {
                self.writeln("Goodbye");
                self.stream.shutdown(Shutdown::Both).unwrap();
                sender.send(Command::Remove{id: self.get_id(), location: room.get_id().to_string()}).unwrap();
            },
            "look" => {
                self.writeln(room.get_name());
                self.writeln("");
                self.writeln(room.get_description());
                self.writeln("");
                self.write("Contents: ");

                for entity in room.entities.values() {
                    match &*entity.borrow() {
                        &Entity::Creature(ref creature) => {
                            self.write(&creature.get_name());
                            self.write(" ");
                        }
                    }
                }

            },
            x => {
                self.writeln(&format!("Unknown command: {}", x))
            }
        }
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();
        match self.stream.read_to_string(&mut line) {
            Ok(_) => line,
            Err(_) => line
        }
    }

    fn write(&mut self, line: &str) {
        self.stream.write(line.as_bytes()).unwrap();
    }

    fn writeln(&mut self, line: &str) {
        self.stream.write(line.as_bytes()).unwrap();
        self.stream.write(b"\r\n").unwrap();
    }
}
