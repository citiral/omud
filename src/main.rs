// while developping allow dead code so the compiler doesn't shit itself as much
#![allow(dead_code)]

mod room;
mod entity;
mod world;
mod creature;
mod command;

use command::*;
use room::*;
use world::*;
use creature::*;

use entity::*;
use std::thread;
use std::io::{self, Read, BufRead, BufReader, Write};
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender};

fn start_listening(ip: &str, sender: Sender<Command>) -> Result<(), io::Error> {
    match TcpListener::bind(ip) {
        Ok(listener) => {
            thread::spawn(move || {
                for mut stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            stream.set_nonblocking(true);

                            // make a player
                            let player = Entity::Creature(Box::new(Player::new(stream)));
                            // add it to the game
                            sender.send(Command::Add{
                                entity: player,
                                location: "spawn".to_string()
                            }).unwrap();
                        },
                        Err(_) => {
                            println!("Failed listening on socket");
                        }
                    }
                }
            });
            Ok(())
        },
        Err(err) => {
            Err(err)
        }
    }
}

fn start_local_dummy_client(ip: String) {
    thread::spawn(move || {
        // connect to the server
        let mut stream = TcpStream::connect(&ip as &str).unwrap();

        // make a thread that only outputs what is received on the stream
        if let Ok(stream) = stream.try_clone() {
            let t = thread::spawn(move || {
                // make a buffered reader and print out the results line by line
                let mut reader = BufReader::new(stream);
                loop {
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(_) => {
                            if line == "" {
                                println!("Closing client read thread.");
                                return;
                            } else {
                                print!("{}", line);
                            }
                        }
                        Err(_) => {
                            println!("Closing client read thread.");
                            return;
                        }
                    }
                }
            });

        } else {
            println!("Failed cloning stream.");
            return;
        }

        // and then continuously send our input to the stream
        let stdin = io::stdin();

        loop {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            line = line.to_string();
            if let Err(_) = stream.write(line.as_bytes()) {
                println!("Closing client write thread.");
                return;
            }
        }
    });
}

fn create_test_world<'a>() -> World {
    let mut world = World::new();

    world.add_room(Room::new("spawn".to_string()));
    world.add_room(Room::new("beep1".to_string()));
    world.add_room(Room::new("beep2".to_string()));

    world
}

fn handle_command(world: &mut World, command: Command) {
    match command {
        Command::Add{entity, location} => {
            if let Some(room) = world.get_room_mut(&location) {
                room.add_entity(entity);
            }
        },
        Command::Remove{id, location} => {
            if let Some(room) = world.get_room_mut(&location) {
                room.remove_entity(id);
            } else {
                println!("Error removing player");
            }
        },
        Command::Move{id, from, to} => {
            let entity = match world.get_room_mut(&from) {
                Some(room) => room.remove_entity(id),
                None => None
            };

            if let Some(e) = entity {
                if let Some(room) = world.get_room_mut(&to) {
                    room.add_entity(e);
                }
            }
        }
    }
}

fn main() {
    let mut world = create_test_world();

    let (sender, receiver) = channel::<Command>();

    if let Err(e) = start_listening("localhost:25565", sender.clone()) {
        println!("Failed hosting server: {}.", e);
        return;
    }


    start_local_dummy_client("localhost:25565".to_string());

    loop {
        // tick the world
        world.tick(sender.clone());

        // handle all events
        loop {
            if let Ok(command) = receiver.try_recv() {
                handle_command(&mut world, command);
            } else {
                break;
            }
        }
    }
}
