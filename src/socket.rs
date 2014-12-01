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


use std::io::TcpStream;

use super::serialize::json;
use super::serialize::json::Json;
use super::serialize::json::{ParseError, DecoderError, DecodeResult};

use super::event::Event;
use super::action::Action;
use super::message::Message;
use super::message::Payload::{Text, Binary};
use super::message::Mask::{TextOp, BinaryOp};


/*
 * Struct representing a websocket
 */
pub struct Socket {
	pub id: String,
	pub stream: TcpStream,
	pub to_event_loop: Sender<Action>,
    pub to_write_task: Sender<Message>,
	pub events: Vec<Event>
}

/*
 * Struct representing rustic-io's JSON message passing schema
 *
 * Encodes to
 * {
 *      event: "EVENT_NAME",
 *      data: {
 *          // Stuff here
 *      }
 * }
 */
#[deriving(Decodable, Encodable)]
pub struct JsonMessage {
    event: String,
    data: String
}


impl Socket {

    /*
     *
     */
    pub fn start(&mut self, from_event_loop: Receiver<Message>) {

        let mut write_stream = self.stream.clone();
        let (fail_sender, fail_receiver): (Sender<uint>, Receiver<uint>) = channel();
        let (write_task_sender, write_task_receiver): (Sender<Message>, Receiver<Message>) = channel();
        self.to_write_task = write_task_sender;

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
                    Ok(msg) => {
                        msg.send(&mut write_stream).unwrap();
                    }
                    Err(e) => { /* Dont care */ }
                }

                // Check for messages from this socket's read task
                match write_task_receiver.try_recv() {
                    Ok(msg) => {
                        msg.send(&mut write_stream).unwrap();
                    }
                    Err(e) => { /* Dont care */ }
                }
            }
        });

        // Blocking read on this stream
        let mut read_stream = self.stream.clone();
        loop {
            match Message::load(&mut read_stream) {
                Ok(msg) => {
                    match msg.payload {
                        Text(json_ptr) => {

                            println!("Received: {}", (*json_ptr).as_slice());

                            let decode_result: DecodeResult<JsonMessage> = json::decode((*json_ptr).as_slice());                            
                            match decode_result {
                                Ok(json_msg) => {
                                    for event in self.events.iter() {
                                        if event.name == json_msg.event {
                                            let data_as_json = json::from_str(json_msg.data.as_slice()).unwrap();
                                            (event.execute)(data_as_json, self.clone());
                                            break;
                                        }
                                    }
                                }
                                Err(e) => {
                                    match e {
                                        DecoderError::ParseError(pe) => {
                                            println!("ParseError decoding received json: {}", pe);
                                        }
                                        DecoderError::ExpectedError(s1, s2) => {
                                            println!("ExpectedError decoding received json...");
                                            println!("s1: {}", s1);
                                            println!("s2: {}", s2);
                                        }
                                        DecoderError::MissingFieldError(s) => {
                                            println!("MissingFieldError decoding received json...");
                                        }
                                        DecoderError::UnknownVariantError(s) => {
                                            println!("UnknownVariantError decoding received json...");
                                        }
                                        DecoderError::ApplicationError(s) => {
                                            println!("ApplicationError decoding received json...");
                                        }
                                    }
                                }
                            }
                        }
                        Binary(bin_ptr) => { /* TODO - Implement */ }
                        Empty => { /* TODO - Implement */ }
                    }
                }
                Err(e) => {
                    fail_sender.send(1);
                    println!("e.desc: {}", e.desc);
                    panic!("Read stream closed");
                }
            }
        }
    }

    /*
     *
     */
    pub fn send(&self, event: &str, data: String) {
        let json_msg = JsonMessage {
            event: String::from_str(event),
            data: data
        };

        let msg = Message {
            payload: Text(box String::from_str(json::encode(&json_msg).as_slice())),
            mask: TextOp
        };

        self.to_write_task.send(msg);
    }

    /*
     *
     */
    pub fn broadcast(&self, event: &str, data: String) {
        let json_msg = JsonMessage {
            event: String::from_str(event),
            data: data
        };

        let msg = Message {
            payload: Text(box String::from_str(json::encode(&json_msg).as_slice())),
            mask: TextOp
        };

        let action = Action {
            event: String::from_str("broadcast"),
            message: msg.clone()
        };
        self.to_event_loop.send(action);
    }
}

impl Clone for Socket {

    /*
     *
     */
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