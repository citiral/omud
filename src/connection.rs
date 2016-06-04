use std::io::{Write, BufRead};
use std::collections::LinkedList;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub struct Connection {
    output: Box<Write>,
    receiver: Receiver<String>
}

impl Connection {
	pub fn new(input: Box<BufRead>, output: Box<Write>) -> Connection {

		let (sender, receiver) = channel();

		thread::spawn(move || {
			loop {
				let mut buffer = String::new();
				input.read_line(&mut buffer);
				sender.send(buffer);
			}
		});

		Connection {
			output: output,
			receiver: receiver
		}
	}

	pub fn read_line(&self) -> Option<String> {
		match self.receiver.try_recv() {
			Ok(val) => Some(val),
			Err(_) => None,
		}
	}
}