use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use std::sync::mpsc::Sender;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Values;

use world::World;
use command::Command;
use room::Room;
use entity::{self, Id, Tick, Describable, Container, Living};
use item::Item;
use output;

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

impl<'a> Container<'a> for Player {
    type Iter = Values<'a, usize, Item>;

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

    fn inventory_iter(&'a self) -> Self::Iter {
        self.inventory.values()
    }
}

impl Living for Player {
    fn get_health(&self) -> f32 {
        self.health
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
                output::echo(&mut*self.stream.borrow_mut(), &parts);
            },
            "status" => {
                output::status(&mut*self.stream.borrow_mut(), self);
            }
            "exit" | "quit" => {
                self.writeln("Goodbye");
                self.stream.borrow_mut().shutdown(Shutdown::Both).unwrap();
                sender.send(Command::Remove{id: self.get_id(), location: room.get_id().to_string()}).unwrap();
            },
            "look" => {
                output::look(&mut*self.stream.borrow_mut(), self, room);
            },
            "go" => {
                if let Some(id) = room.get_exit(parts[1]) {
                    sender.send(Command::Move{id: self.get_id(), from: room.get_id().to_string(), to: id}).unwrap();
                } else {
                    self.writeln(&format!("There is no exit named {}.", parts[1]));
                }

            }
            "i" | "inventory" => {
                self.writeln("Inventory: ");
                for i in self.inventory_iter() {
                    self.writeln(&format!("Item: {}", &i.get_name()));
                }
            }
            _ => {
                output::unknown_command(&mut*self.stream.borrow_mut(), command);
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
