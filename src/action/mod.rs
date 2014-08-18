// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use std::str;
use super::server::socket::Socket;

#[path="../socket/mod.rs"]
mod socket;

pub struct Action<'a> {
    pub event: String,
    pub socket: Socket<'a>
}

impl<'a> Action<'a> {
    pub fn new(event: &str, socket: Socket) -> Action<'a> {
        Action {
            event: String::from_str(event),
            socket: socket
        }
    }
}
