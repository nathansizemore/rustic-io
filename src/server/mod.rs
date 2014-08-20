// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.

extern crate serialize;

use serialize::json;
use self::socket::Socket;
use self::event::Event;
use super::action::Action;
use super::message::Message;
use super::message::{Message, TextOp, Text, BinaryOp, Binary};

#[path="../socket/mod.rs"]
pub mod socket;

#[path="../event/mod.rs"]
pub mod event;

// #[path="../action/mod.rs"]
// pub mod action;


pub struct Server<'a> {
    pub sockets: Vec<Socket<'a>>,
    pub events: Vec<Event<'a>>,
    pub to_event_loop: Sender<Action<'a>>,
    pub socket_id: String
}

impl<'a> Server<'a> {
    pub fn new() -> Server<'a> {
        let (tx, rx): (Sender<Action>, Receiver<Action>) = channel();
        Server {
            sockets: Vec::new(),
            events: Vec::new(),
            to_event_loop: tx.clone(),
            socket_id: String::from_str("")
        }
    }

    pub fn on(&mut self, event_name: &str, execute: fn(data: json::Json, server: Server)) {
        self.events.push(Event::new(event_name, execute));
    }

    pub fn send(&self, msg: Message) {

        // Find the socket, dunno why I made this so fucking dumb...
        let mut to_socket: Socket;
        println!("server.sockets.length: {}", self.sockets.len());
        for socket in self.sockets.iter() {
            if socket.id == self.socket_id {
                to_socket = socket.clone();
                let action = Action {
                    event: String::from_str("send"),
                    socket: to_socket,
                    message: msg.clone()
                };

                // Send it to the event_loop to write out
                println!("Sending data to event loop");
                self.to_event_loop.send(action);
            }
        }        
    }
}

impl<'a> Clone for Server<'a> {
    fn clone(&self) -> Server<'a> {
        Server {
            sockets: self.sockets.clone(),
            events: self.events.clone(),
            to_event_loop: self.to_event_loop.clone(),
            socket_id: self.socket_id.clone()
        }
    }
}



