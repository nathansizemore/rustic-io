// Copyright (c) 2014 Nathan Sizemore

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


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
 */
pub struct Socket {

	// Sec-Websocket-Accept
	pub id: String,

	// Reference to stream
	pub stream: TcpStream,

	// Channel to event loop's receiver
	pub to_event_loop: Sender<Action>,

	// Vector of events listening for
	pub events: Vec<Event>
}



impl Socket {

    /*
     *
     */
    pub fn start(&self, from_event_loop: Receiver<Message>) {

        let mut write_stream = self.stream.clone();
        let (fail_sender, fail_receiver): (Sender<uint>, Receiver<uint>) = channel();
        let (write_task_sender, write_task_receiver): (Sender<Message>, Receiver<Message>) = channel();
        spawn(proc() {
            loop {

                // Check for fail message
                match fail_receiver.try_recv() {
                    Ok(kill) => {
                        panic!("Write stream closed");
                    }
                    Err(e) => { /* Dont care */ }
                }

                // Check for messages from event loop
                match from_event_loop.try_recv() {
                    Ok(kill) => {
                        panic!("Write stream closed");
                    }
                    Err(e) => { /* Dont care */ }
                }

                // Check for messages from this socket's read task
                match write_task_receiver.try_recv() {
                    Ok(kill) => {
                        panic!("Write stream closed");
                    }
                    Err(e) => { /* Dont care */ }
                }

            }
        });
    }










	/*
     * "send" Action to the event loop
     * id held in self.socket_id
     *
     * Probably not the best way, but Im retarded and did it
     */
    pub fn send(&self, event: &str, data: String) {
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