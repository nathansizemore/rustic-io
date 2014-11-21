// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use std::io::{TcpStream};
use super::serialize::json;
use self::action::Action;
use self::message::Message;
use self::message::Payload::{Text, Binary};
use self::message::Mask::{TextOp, BinaryOp};
use self::event::Event;


#[path="../event/mod.rs"]
pub mod event;

#[path="../action/mod.rs"]
pub mod action;

#[path="../message/mod.rs"]
pub mod message;

/*
 * Struct representing a websocket
 * id is the returned Sec-Socket-Key in return header
 */
pub struct Socket {

	// Sec-Websocket-Accept
	pub id: String,

	// Reference to stream
	pub stream: TcpStream,

	// Channel to event loop's receiver
	pub to_event_loop: Sender<Action>,

    // Channel to this socket's write task receiver
    pub to_write_task: Sender<Message>,

	// Vector of events listening for
	pub events: Vec<Event>
}

/*
 * Struct representing rustic-io's JSON message passing
 *
 * Encodes to
 * {
 * 		event: "EVENT_NAME",
 * 		data: {
 * 			// Stuff here
 * 		}
 * }
 */
#[deriving(Decodable, Encodable)]
struct JsonMessage {

	// Name of event being passed
    event: String,

    // json::encode() version of some struct
    data: String
}

impl Socket {

	/*
     * "send" Action to the event loop
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

        // Send it out
        self.to_write_task.send(msg);
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

        let action = Action {
            event: String::from_str("broadcast"),
            socket: self.clone(),
            message: msg.clone()
        };
        self.to_event_loop.send(action);
    }
}

impl Clone for Socket {
	fn clone(&self) -> Socket {
		Socket {
			id: self.id.clone(),
			stream: self.stream.clone(),
            to_event_loop: self.to_event_loop.clone(),
            to_write_task: self.to_write_task.clone(),
            events: self.events.clone()
		}
	}
}