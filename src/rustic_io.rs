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
extern crate collections;

use std::str;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use self::serialize::json;
use self::serialize::json::Json;
use self::collections::treemap::TreeMap;

use self::socket::Socket;
use self::socket::event::Event;
use self::socket::action::Action;
use self::socket::message::{Message, Text, Binary};
use self::server::Server;
use self::httpheader::{RequestHeader, ReturnHeader};
use self::socketmessenger::SocketMessenger;

pub mod socket;
pub mod server;
mod httpheader;
mod socketmessenger;

/*
 * Returns a new Server
 */
pub fn bind(ip: &str, port: u16) -> Server {
    Server {
        ip: String::from_str(ip),
        port: port.clone(),
        events: Vec::new()
    }    
}

/*
 * Starts the Event Loop and TCP/IP Server
 */
pub fn start(server: Server) {
    // Event loop communication pipe
    let (sender, receiver): (Sender<Action>, Receiver<Action>) = channel();

    /*
     * Event Loop Task
     *
     * Needs:
     *  - Receiver (Closure captured)
     *  - Sender (Sockets need to pass messages back into this loop)
     *  - Vector of events to listen for
     */
    let sender_clone = sender.clone();
    let event_list = server.events.clone();    
    spawn(proc() {
        event_loop(receiver, sender_clone, event_list)
    });

    /*
     * TCP/IP Server
     *
     * Intended to serve forever
     */
    let listener = TcpListener::bind(server.ip.as_slice(), server.port);
    let mut acceptor = listener.listen();
    for stream in acceptor.incoming() {
        match stream {
            Ok(stream) => {
                /*
                 * Websocket Accept Task
                 *
                 * Needs:
                 *  - TCPStream
                 *  - Sender (To event loop)
                 */
                let to_event_loop = sender.clone();
                spawn(proc() {                
                    process_new_connection(stream, to_event_loop)
                })
            }
            Err(e) => {
                println!("Error accepting connection: {}", e)
            }            
        }
    }

    // If we get here, drop resources to fds
    drop(acceptor);
}

/*
 * Accepts and parses input stream from a new connection
 * looking for the Sec-WebSocket-Key header in the HTTP/1.1 Request
 *
 * Executed on separate thread.  Exits if the header is not found
 */
fn process_new_connection(mut stream: TcpStream, sender: Sender<Action>) {
    let mut buffer = [0, ..1024]; // TODO - Determine a header size based on modern borwsers
    match stream.read(buffer) {
        Ok(result) => {
            //println!("Ok: {}", result)
        }
        Err(e) => {
            println!("Error reading incoming connection buffer: {}", e)
            return;
        }
    }
    
    // Parse request for Sec-WebSocket-Key
    match str::from_utf8(buffer) {
        Some(header) => {
            let request_header = RequestHeader::new(header);
            if request_header.is_valid() {
                let return_header = ReturnHeader::new_accept(request_header.sec_websocket_key.as_slice());
                match stream.write(return_header.to_string().as_bytes()) {
                    Ok(result) => {
                        //println!("New connection");
                        // Create new socket
                        let socket = Socket {
                            id: String::from_str(return_header.sec_websocket_accept.as_slice()),
                            stream: stream,
                            to_event_loop: sender.clone(),
                            events: Vec::new()
                        };

                        // Tell event loop
                        let action = Action::new("new_connection", socket);
                        sender.send(action);
                    }
                    Err(e) => {
                        println!("Error writing to stream: {}", e)
                    }
                }
            } else {
                println!("Request header invalid");
            }
        }
        None => {
            println!("Buffer not valid UTF-8")
        }
    }
}

/*
 * Event Loop
 *     - Listens for new sockets from TCP server
 *     - Listens for new events received from sockets
 */
fn event_loop(receiver: Receiver<Action>, sender: Sender<Action>, events: Vec<Event>) {
    // Vector of socket ids and senders to each of their listen tasks
    let mut socket_msgers: Vec<SocketMessenger> = Vec::new();
    
    loop {
        // Non-blocking message receive loop
        match receiver.try_recv() {
            Ok(action) => {
                match action.event.as_slice() {
                    "new_connection" => {
                        /*
                         * New Socket Task
                         *
                         * Needs:
                         *  - Clone of the socket (for it's stream)
                         *  - Sender (For writes on the socket)
                         *  - Receiver (To receive write events from event loop)
                         */
                        let (socket_sender, socket_receiver): (Sender<Message>, Receiver<Message>) = channel();
                        let socket = Socket {
                            id: action.socket.id.clone(),
                            stream: action.socket.stream.clone(),
                            to_event_loop: sender.clone(),
                            events: events.clone()
                        };

                        // Add socket and channel to messenger vector
                        socket_msgers.push(SocketMessenger {
                            id: socket.id.clone(),
                            to_socket: socket_sender.clone()
                        });

                        // Start the socket task
                        spawn(proc() {
                            start_new_socket(socket, socket_receiver)
                        });
                    }
                    "drop_connection" => {

                    }
                    "broadcast" => {
                        // Send the message to everyone
                        for msger in socket_msgers.iter() {
                            msger.to_socket.send(action.message.clone())
                        }
                    }
                    "send" => {
                        // Find the handler to this socket's out stream
                        for msger in socket_msgers.iter() {
                            if msger.id == action.socket.id {
                                msger.to_socket.send(action.message.clone())
                            }
                        }
                    }
                    _ => {
                        println!("Event loop received unknown action: {}", action.event);
                    }
                }
            }
            Err(e) => {
                // Do nothing.
                // try_recv() returns Err when no message is available
            }
        }
    }
}

/*
 * Starts the I/O process for a new socket connection
 */
fn start_new_socket(socket: Socket, receiver: Receiver<Message>) {

    /*
     * Socket Write Task
     *
     * Needs
     *  - Stream copy (For non-blocking i/o)
     *  - Receiver (For messages from event loop)
     *  - Receiver (For fail signal (If read task gets EOF))
     */
    let mut stream_write = socket.stream.clone();
    let (sender, fail_receiver): (Sender<u16>, Receiver<u16>) = channel();
    spawn(proc() {
        loop {
            // Look for a signal to fail task
            match fail_receiver.try_recv() {
                Ok(kill) => {
                    fail!("Write stream closed");
                }
                Err(e) => {
                    // Do nothing, no signal is a good thing
                }
            }
            // Look for a message from the event loop
            match receiver.try_recv() {
                Ok(msg) => {
                    msg.send(&mut stream_write).unwrap();
                }
                Err(e) => {
                    // Do nothing
                }
            }
        }
    });

    // Open up a blocking read on this socket
    let mut stream_read = socket.stream.clone();
    loop {
        match Message::load(&mut stream_read) {
            Ok(msg) => {
                match msg.payload {
                    Text(ptr) => {
                        let json_slice = (*ptr).as_slice();
                        //println!("Socket: {} recevied: {}", socket.id, json_slice);
                        parse_json(json_slice, socket.clone());
                    }
                    Binary(ptr) => {
                        // TODO - Do awesome binary shit
                    }
                    Empty => {
                        // TODO - Implement close to write stream
                    }
                }
            }
            Err(e) =>{
                println!("e.desc: {}", e.desc);
                if e.desc == "end of file" {
                    sender.send(1);
                    fail!("Read stream closed");
                }
            }
        }
    }    
}

/*
 * Decodes received JSON
 * Expects the JSON to be in the following format:
 *     {
 *         "event": "SOME_STRING",
 *         "data": {
 *             // Important data stuff
 *         }
 *     }
 */
fn parse_json(json_data: &str, socket: Socket) {
    match json::from_str(json_data) {
        Ok(result) => {
            //println!("JSON decoded as: {}", result)

            // Try and parse Json as object
            match result.as_object() {
                Some(object) => {
                    // Get passed event
                    match try_find_event(object) {
                        Some(event) => {
                            let data = get_json_data(object);
                            for listening_for in socket.events.iter() {
                                if event == listening_for.name {
                                    (listening_for.execute)(data, socket.clone());
                                    break;
                                }
                            }
                        }
                        None => {
                            println!("Error finding event key")
                            return;
                        }
                    }
                }
                None => {
                    println!("Error decoding Json as object")
                }
            }
        }
        Err(e) => {
            println!("Error deserializing json: {}", e)
        }
    }
}

/*
 * Attempts to find the value for the "event" key
 * in the passed Json.
 * If not present, or cannot be parsed as a string,
 * returns None
 */
fn try_find_event(treemap: &TreeMap<String, Json>) -> Option<String> {
    match treemap.find(&String::from_str("event")) {
        Some(value) => {
            if value.is_string() {
                return Some(String::from_str(value.as_string().unwrap()));
            }
            None // Value was not a string
        }
        None => {
            None // Event key not found in Json
        }
    }
}

/*
 * Attempts to find and return the value for the "data" key in 
 * the passed Json
 * If not found, or not able to parse into object, returns 
 * an empty Json object (e.g. "{}")
 */
fn get_json_data(treemap: &TreeMap<String, Json>) -> Json {
    match treemap.find(&String::from_str("data")) {
        Some(value) => {
            if value.is_object() {
                return value.clone();
            } else {
                // Send back empty Json
                let no_data = json::from_str("{}").unwrap();
                return no_data;
            }
        }
        None => {
            // Send back empty Json
            let no_data = json::from_str("{}").unwrap();
            return no_data;
        }
    }
}

