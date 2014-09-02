// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use self::event::Event;
use self::socket::Socket;
use super::action::Action;
use super::serialize::json;
use super::serialize::json::Json;
use super::message::{Message, TextOp, Text, BinaryOp, Binary};


#[path="../socket/mod.rs"]
pub mod socket;

#[path="../event/mod.rs"]
pub mod event;


/*
 * Struct representing a socket server
 */
pub struct Server {
    pub sockets: Vec<Socket>,
    pub events: Vec<Event>,
    pub to_event_loop: Sender<Action>,
    pub socket_id: String
}

/*
 * Struct representing rustic-io's JSON message passing
 */
#[deriving(Decodable, Encodable)]
struct JsonMessage {
    event: String,
    data: String // json::encode() value expected
}

impl Server {
    // Constructs a new Server
    pub fn new() -> Server {
        let (tx, rx): (Sender<Action>, Receiver<Action>) = channel();
        Server {
            sockets: Vec::new(),
            events: Vec::new(),
            to_event_loop: tx.clone(),
            socket_id: String::from_str("")
        }
    }

    // Adds the passed function to the execute vector
    pub fn on(&mut self, event_name: &str, execute: fn(data: Json, server: Server)) {
        self.events.push(Event::new(event_name, execute));
    }

    /*
     * Sends the passed message to the current socket
     * id held in self.socket_id
     *
     * Probably not the best way, but Im retarded and did it
     */
    pub fn send(&self, event: &str, data: String) {
        // Build a JsonMessage
        let json_msg = JsonMessage {
            event: String::from_str(event),
            data: data
        };

        // Wrap it in the WebSocket bitmask
        let msg = Message {
            payload: Text(box String::from_str(json::encode(&json_msg).as_slice())),
            mask: TextOp
        };

        // Send the message
        let mut to_socket: Socket;
        for socket in self.sockets.iter() {
            if socket.id == self.socket_id {
                to_socket = socket.clone();
                let action = Action {
                    event: String::from_str("send"),
                    socket: to_socket,
                    message: msg.clone()
                };
                self.to_event_loop.send(action);
                break; // Leave loop when socket is found
            }
        }
    }

    /*
     * Sends the passed message to all currently connected sockets
     */
    pub fn broadcast(&self, event: &str, data: String) {
        // Build a JsonMessage
        let json_msg = JsonMessage {
            event: String::from_str(event),
            data: data
        };

        // Wrap it in the WebSocket bitmask
        let msg = Message {
            payload: Text(box String::from_str(json::encode(&json_msg).as_slice())),
            mask: TextOp
        };

        // Ensure we actually have at least one person to send this to
        if self.sockets.len() > 0 {
            let action = Action {
                event: String::from_str("broadcast"),
                socket: self.sockets[0].clone(),
                message: msg.clone()
            };
            self.to_event_loop.send(action);
        } else {
            println!("There is no one connected right now...")
        }
    }
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            sockets: self.sockets.clone(),
            events: self.events.clone(),
            to_event_loop: self.to_event_loop.clone(),
            socket_id: self.socket_id.clone()
        }
    }
}



