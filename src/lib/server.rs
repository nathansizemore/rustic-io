// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use self::socket::Socket;
use self::event::Event;

#[path="./socket.rs"]
pub mod socket;

#[path="./event.rs"]
pub mod event;


pub struct Server<'a> {
    pub sockets: Vec<Socket<'a>>,
    pub events: Vec<Event<'a>>
}

impl<'a> Server<'a> {
    pub fn new() -> Server<'a> {
        Server {
            sockets: Vec::new(),
            events: Vec::new()    
        }
    }

    pub fn on(&self, event_name: &str, execute: &fn(data: &str, server: Server)) {

    }
}

impl<'a> Clone for Server<'a> {
    fn clone(&self) -> Server<'a> {
        Server {
            sockets: self.sockets.clone(),
            events: self.events.clone()
        }
    }
}



