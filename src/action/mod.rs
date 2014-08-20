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
use super::message::Message;
use super::message::{Message, TextOp, Text, BinaryOp, Binary};

pub struct Action<'a> {
    pub event: String,
    pub socket: Socket<'a>,
    pub message: Message
}

impl<'a> Action<'a> {
    pub fn new(event: &str, socket: Socket) -> Action<'a> {
        // Build a default message
        let (payload, opcode) = (Text(box String::from_str("blah, blah")), TextOp);
        let msg = Message {
            payload: payload,
            opcode: opcode
        };

        Action {
            event: String::from_str(event),
            socket: socket,
            message: msg.clone()
        }
    }
}
