use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write};
use world::World;
use std::sync::mpsc::Sender;
use command::Command;

pub trait Tick {
    fn tick(&mut self, world: &World, sender: Sender<Command>);
}

pub trait Creature: Tick {
    fn get_health(&self) -> f32;
}

pub struct Player {
    health: f32,
    connected: bool,
    stream: TcpStream,
}

impl Creature for Player {
    fn get_health(&self) -> f32 {
        self.health
    }
}

impl Tick for Player {
    fn tick(&mut self, world: &World, sender: Sender<Command>) {
        let line = self.read_line();

        if line != "" {
            self.handle_command(world, sender, &line)
        }
    }
}

impl Player {
    pub fn new(stream: TcpStream) -> Player {
        Player {
            health: 100.0,
            connected: false,
            stream: stream,
        }
    }

    fn handle_command(&mut self, world: &World, sender: Sender<Command>, command: &str) {
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
                self.stream.shutdown(Shutdown::Both);
            },
            "look" => {

            },
            x => {
                self.writeln(&format!("Unknown command: {}", x))
            }
        }
    }

    fn read_line(&mut self) -> String {
        let mut line = String::new();
        self.stream.read_to_string(&mut line);
        line
    }

    fn write(&mut self, line: &str) {
        self.stream.write(line.as_bytes()).unwrap();
    }

    fn writeln(&mut self, line: &str) {
        self.stream.write(line.as_bytes()).unwrap();
        self.stream.write(b"\r\n").unwrap();
    }
}
