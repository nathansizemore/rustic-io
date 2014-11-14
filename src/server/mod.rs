// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use super::socket::event::Event;
use super::socket::Socket;
use super::serialize::json::Json;


/*
 * Struct representing a socket server
 */
pub struct Server {
    pub ip: String,
    pub port: String,
    pub events: Vec<Event>
}

impl Server {

    // Adds the passed function to the execute vector
    pub fn on(&mut self, event_name: &str, execute: fn(data: Json, socket: Socket)) {
        self.events.push(Event::new(event_name, execute));
    }
}

impl Clone for Server {
    fn clone(&self) -> Server {
        Server {
            ip: self.ip.clone(),
            port: self.port.clone(),
            events: self.events.clone()
        }
    }
}
