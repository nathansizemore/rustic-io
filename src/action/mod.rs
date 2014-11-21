// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use super::Socket;
use super::message::Message;
use super::message::Payload::{Text, Binary};
use super::message::Mask::{TextOp, BinaryOp};


/*
 * Struct representing an Action the Event Loop needs to execute
 *
 * Current supported actions:
 *  - "new_connection"
 *  - "drop_connection"
 *  - "broadcast"
 *  - "send"
 */
pub struct Action {
    pub event: String,
    pub socket: Socket,
    pub message: Message
}

impl Action {

    // Constructs a new action
    pub fn new(event: &str, socket: Socket) -> Action {
        // Build a default message for when action does not need a message
        let (payload, mask) = (Text(box String::from_str("blah, blah")), TextOp);
        let msg = Message {
            payload: payload,
            mask: mask
        };

        Action {
            event: String::from_str(event),
            socket: socket,
            message: msg.clone()
        }
    }
}
