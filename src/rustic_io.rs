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


extern crate serialize;
extern crate collections;

use std::str;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use serialize::json;
use serialize::json::Json;
use collections::tree_map::TreeMap;

use socket::Socket;
use event::Event;
use action::Action;
use message::Message;
use message::Payload::{Text, Binary};
use server::Server;
use httpheader::{RequestHeader, ReturnHeader};
use socketmessenger::SocketMessenger;


/*
 * Returns a new Server
 */
pub fn new_server(ip: &str, port: &str) -> Server {
    return Server::new(ip, port);
}

/*
 * Starts the Event Loop and TCP Server
 */
pub fn start(server: Server) {

    // Start the event loop
    let (action_sender, action_receiver): (Sender<String>, Receiver<String>) = channel();
    let (new_conn_sender, new_conn_receiver): (Sender<TcpStream>, Receiver<TcpStream>) = channel();

    let event_list = server.events.clone();
    spawn(proc() {
        event_loop::start(action_sender, action_receiver, new_conn_receiver, event_list)
    });

    // Start TCP Server
    let mut address = String::new();
    address.push_str(server.ip.as_slice())
        .push_str(":")
        .push_str(server.port.as_slice());

    let listener = TcpListener::bind(address.as_slice());
    let mut acceptor = listener.listen();
    for stream in acceptor.incoming() {
        match stream {
            Ok(stream) => {
                spawn(proc() {
                    process_new_tcp_connection(stream, new_conn_sender)
                })
            }
            Err(e) => {
                println!("Error accepting incoming tcp connection...");
                println!("{}", e);
            }
        }
    }
    drop(acceptor);
}

/*
 * Grabs the request from the stream and attempts to parse as an HTTP WebSocket Request header
 * If successful, sends the stream to the event to start a connection
 * Fails silently
 */
fn process_new_tcp_connection(mut stream: TcpStream, new_conn_sender: Sender<TcpStream>) {
    let mut buffer = [0u8, ..512]; // Incoming HTTP header size
    stream.read(&mut buffer).unwrap();

    match str::from_utf8(buffer.as_slice()) {
        Some(header) => {
            let request_header = RequestHeader::new(header);
            if (request_header.is_valid()) {
                let return_header = ReturnHeader::new(request_header.sec_websocket_key.as_slice());
                match stream.write(return_header.to_string().as_bytes()) {
                    Ok(result) => {
                        new_conn_sender.send(stream.clone());
                    }
                    Err(e) => {
                        // User must have closed connection
                    }
                }
            }
        }
        None => {
            // Buffer wasn't valid UTF-8
        }
    }
}


/*
 * Starts the I/O process for a new socket connection
 */
fn start_new_socket(mut socket: Socket, broadcast_receiver: Receiver<Message>) {

    /*
     * Socket Write Task
     *
     * Needs
     *  - Stream copy (For non-blocking i/o)
     *  - Receiver (For messages from event loop)
     *  - Receiver (For fail signal (If read task gets EOF))
     *  - Receiver (From socket read stream)
     */
    let mut stream_write = socket.stream.clone();
    let (fail_sender, fail_receiver): (Sender<u16>, Receiver<u16>) = channel();
    let (socket_sender, send_receiver): (Sender<Message>, Receiver<Message>) = channel();
    spawn(proc() {
        loop {
            match fail_receiver.try_recv() {
                Ok(kill) => {
                    panic!("Write stream closed");
                }
                Err(e) => { /* Dont care */ }
            }
            
            match broadcast_receiver.try_recv() {
                Ok(msg) => {
                    msg.send(&mut stream_write).unwrap();
                }
                Err(e) => { /* Dont care */ }
            }

            match send_receiver.try_recv() {
                Ok(msg) => {
                    msg.send(&mut stream_write).unwrap();
                }
                Err(e) => { /* Dont care */ }
            }
        }
    });

    // Open up a blocking read on this socket
    socket.to_write_task = socket_sender;
    let mut stream_read = socket.stream.clone();
    loop {
        match Message::load(&mut stream_read) {
            Ok(msg) => {
                match msg.payload {
                    Text(ptr) => {
                        let json_slice = (*ptr).as_slice();
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
                if e.desc == "end of file" {
                    fail_sender.send(1);
                    panic!("Read stream closed");
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
            match result.as_object() {
                Some(object) => {
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
    match treemap.get(&String::from_str("event")) {
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
    match treemap.get(&String::from_str("data")) {
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

