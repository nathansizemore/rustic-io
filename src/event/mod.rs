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

/*
 * Struct representing an event, and a function to execute
 * when that event is received from the client
 */
pub struct Event {
    pub name: String,
    pub execute: fn(data: json::Json, server: super::Server)
}

impl Event {

    // Constructs an Event object
    pub fn new(event: &str, execute: fn(data: json::Json, server: super::Server)) -> Event {
        Event {
            name: String::from_str(event),
            execute: execute
        }
    }
}

impl Clone for Event {
    fn clone(&self) -> Event {
        Event {
            name: self.name.clone(),
            execute: self.execute
        }
    }
}
