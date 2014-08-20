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

use std::str;
use serialize::json;


pub struct Event<'a> {
    pub name: String,
    pub execute: fn(data: json::Json, server: super::Server)
}

impl<'a> Event<'a> {

    // Constructs an Event object
    pub fn new(event: &str, execute: fn(data: json::Json, server: super::Server)) -> Event<'a> {
        Event {
            name: String::from_str(event),
            execute: execute
        }
    }
}

impl<'a> Clone for Event<'a> {
    fn clone(&self) -> Event<'a> {
        Event {
            name: self.name.clone(),
            execute: self.execute
        }
    }
}
