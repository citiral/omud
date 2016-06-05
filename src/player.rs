use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use world::World;
use std::sync::mpsc::Sender;
use command::Command;
use room::Room;
use entity::{self, Entity, Id, Tick, Describable};
use std::cell::RefCell;
use creature::Creature;

pub struct Player {
    id: usize,
    name: String,
    health: f32,
    connected: bool,
    stream: RefCell<TcpStream>,
}

impl Creature for Player {
    fn get_health(&self) -> f32 {
        self.health
    }
}

impl Id for Player {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Tick for Player {
    fn tick(&self, room: &Room, world: &World, sender: Sender<Command>) {
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
            stream: RefCell::new(stream),
        }
    }

    fn handle_command(&self, room: &Room, world: &World, sender: Sender<Command>, command: &str) {
        let parts: Vec<&str> = command.splitn(2, ' ').collect();

        match parts[0] {
            "echo" => {
                if parts.len() > 1 {
                    self.writeln(parts[1]);
                } else {
                    self.writeln(parts[0]);
                }
            },
            "status" => {
                // TODO reformat this to a generic function and get status of other players and creatures
                self.write("Status of ");
                self.write(&self.name);
                self.writeln("");
                self.write("health: ");
                self.write(&format!("{}", self.health));
                self.writeln("");
            }
            "exit" | "quit" => {
                self.writeln("Goodbye");
                self.stream.borrow_mut().shutdown(Shutdown::Both).unwrap();
                sender.send(Command::Remove{id: self.get_id(), location: room.get_id().to_string()}).unwrap();
            },
            "look" => {
                self.writeln(room.get_name());
                self.writeln("");
                self.writeln(room.get_description());
                self.writeln("");
                self.write("Contents: ");

                for entity in room.entities.values() {
                    if let Some(describable) = entity.as_describable() {
                        if entity.get_id() == self.id {
                            self.write("you ");
                        } else {
                            self.write(&describable.get_name());
                            self.write(" ");
                        }
                    }
                }

                self.writeln("");

                self.write("Exits: ");
                for &(ref name, _) in room.exits() {
                    self.write(&name);
                    self.write(" ");
                }

                self.writeln("");

            },
            "go" => {
                if let Some(id) = room.get_exit(parts[1]) {
                    sender.send(Command::Move{id: self.get_id(), from: room.get_id().to_string(), to: id}).unwrap();
                } else {
                    self.writeln(&format!("There is no exit named {}.", parts[1]));
                }

            }
            x => {
                self.writeln(&format!("Unknown command: {}", x))
            }
        }
    }

    fn read_line(&self) -> String {
        let mut line = String::new();
        match self.stream.borrow_mut().read_to_string(&mut line) {
            Ok(_) => line,
            Err(_) => line
        }
    }

    fn write(&self, line: &str) {
        self.stream.borrow_mut().write(line.as_bytes()).unwrap();
    }

    fn writeln(&self, line: &str) {
        self.stream.borrow_mut().write(line.as_bytes()).unwrap();
        self.stream.borrow_mut().write(b"\r\n").unwrap();
    }
}
