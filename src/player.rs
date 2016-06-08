use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::cell::RefCell;
use std::collections::HashMap;

use world::World;
use command::Command;
use room::Room;
use entity::{self, Id, Tick, Describable, Container};
use item::Item;

pub struct Player {
    id: usize,
    name: String,
    health: f32,
    connected: bool,
    stream: RefCell<TcpStream>,
    inventory: HashMap<usize, Item>,
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
            inventory: HashMap::new(),
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
                self.writeln(&room.name);
                self.writeln("");
                self.writeln(&room.description);
                self.writeln("");

                self.write("Creatures here: ");
                for creature in room.creatures.values() {
                    if let Some(describable) = creature.as_describable() {
                        if creature.get_id() == self.id {
                            self.write("you ");
                        } else {
                            self.write(&describable.get_name());
                            self.write(" ");
                        }
                    }
                }
                self.writeln("");

                if !room.items.is_empty() {
                    self.write("Items here: ");
                    for item in room.items.values() {
                        self.write(&item.get_name());
                    }
                    self.writeln("");
                }

                /*for entity in room.entities.values() {
                    if let Some(describable) = entity.as_describable() {
                        if entity.get_id() == self.id {
                            self.write("you ");
                        } else {
                            self.write(&describable.get_name());
                            self.write(" ");
                        }
                    }
                }*/

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

impl Container for Player {
    fn has_item(&self, id: usize) -> bool {
        self.inventory.contains_key(&id)
    }

    fn add_item(&mut self, item: Item) {
        self.inventory.insert(item.get_id(), item);
    }

    fn get_item(&self, id: usize) -> Option<&Item> {
        self.inventory.get(&id)
    }

    fn get_item_mut(&mut self, id: usize) -> Option<&mut Item> {
        self.inventory.get_mut(&id)
    }

    fn remove_item(&mut self, id: usize) -> Option<Item> {
        self.inventory.remove(&id)
    }

    fn item_count(&self) -> usize {
        self.inventory.len()
    }
}