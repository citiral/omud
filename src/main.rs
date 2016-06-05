// while developping allow dead code so the compiler doesn't shit itself as much
#![allow(dead_code)]

extern crate time;

mod room;
mod entity;
mod world;
mod creature;
mod command;
mod player;
mod item;

use player::*;
use command::*;
use room::*;
use world::*;

use item::*;
use entity::*;
use std::thread;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender};
use time::{Duration, PreciseTime};


fn start_listening(ip: &str, sender: Sender<Command>) -> Result<(), io::Error> {
    match TcpListener::bind(ip) {
        Ok(listener) => {
            thread::spawn(move || {
                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            stream.set_nonblocking(true).unwrap();

                            // make a player
                            let player = Entity::Player(Player::new(stream));
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
            thread::spawn(move || {
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
                                print!("> {}", line);
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

    let mut r1 = Room::new("spawn".to_string());
    let mut r2 = Room::new("beep1".to_string());
    let mut r3 = Room::new("beep2".to_string());

    r1.connect_to_room(&mut r2, "north".to_string(), "south".to_string());

    r2.add_entity(Entity::Thing(Thing::new("gold".to_string(), "gold coin".to_string(), 1)));

    world.add_room(r1);
    world.add_room(r2);
    world.add_room(r3);



    world
}

fn main() {
    let mut world = create_test_world();

    let (sender, receiver) = channel::<Command>();

    if let Err(e) = start_listening("localhost:25565", sender.clone()) {
        println!("Failed hosting server: {}.", e);
        return;
    }


    start_local_dummy_client("localhost:25565".to_string());

    let mut now = PreciseTime::now();
    let steplength = Duration::milliseconds(1000/20);

    loop {

        let elapsed = now.to(PreciseTime::now());

        if elapsed > steplength {
        // tick the world
            now = PreciseTime::now();
            world.tick(sender.clone());
        } else {
            let remaining = steplength - elapsed;
            //println!("sleeping for {}", remaining.num_milliseconds());
            thread::sleep(std::time::Duration::from_millis(remaining.num_milliseconds() as u64));
            continue;
        }

        // handle all events
        loop {
            if let Ok(command) = receiver.try_recv() {
                if let Err(e) = command.execute(&mut world) {
                    println!("Error executing command: {}", e);
                }
            } else {
                break;
            }
        }
    }
}
