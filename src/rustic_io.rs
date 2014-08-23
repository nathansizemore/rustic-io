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
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use self::server::Server;
use self::action::Action;
use self::server::socket::Socket;
use self::server::event::Event;
use self::message::{Message, TextOp, Text, BinaryOp, Binary};
use self::serialize::json;
use self::socketmessenger::SocketMessenger;
use self::httpheader::{RequestHeader, ReturnHeader};

mod httpheader;
mod action;
mod socketmessenger;
pub mod message;
pub mod server;

#[crate_id = "rustic-io"]

pub fn start(server: Server, ip: &str, port: u16) {

    /*
     * Communication channel
     *     - From HTTP Server to Event Loop (Action Passed)
     */
    let (to_event_loop, from_conn_pool): (Sender<Action>, Receiver<Action>) = channel();

    // Start up event loop
    let mut server_clone = server.clone();
    let mut to_event_loop_clone = to_event_loop.clone();
    spawn(proc() {
        event_loop(server_clone, from_conn_pool, to_event_loop_clone)
    });

    // Start TCP server
    let listener = TcpListener::bind(ip, port);
    let mut acceptor = listener.listen();
    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {
                println!("Error: {}", e)

                // TODO - Handle errors and shit
            }
            Ok(stream) => {
                let event_loop_msgr = to_event_loop.clone();
                spawn(proc() {                
                    process_new_connection(stream, event_loop_msgr)
                })
            }
        }
    }

    // Close resources when loop stops/fails or some shit I need to figure out
    drop(acceptor);
}

/*
 * Accepts and parses input stream from a new connection
 * looking for the Sec-WebSocket-Key header in the HTTP/1.1 Request
 *
 * Executed on separate thread.  Exits if the header is not found
 */
fn process_new_connection(mut stream: TcpStream, to_conn_pool: Sender<Action>) {
    let mut buffer = [0, ..1024]; // 512 may be fine here?
    match stream.read(buffer) {
        Ok(result) => {
            println!("Ok: {}", result)
        }
        Err(e) => {
            println!("Error: {}", e)
            // TODO - Handle errors and shit
        }
    }
    
    //Parse request for Sec-WebSocket-Key
    match str::from_utf8(buffer) {
        Some(header) => {
            println!("{}", header);
            let request_header = RequestHeader::new(header);
            if request_header.is_valid() {
                let return_header = ReturnHeader::new_accept(request_header.sec_websocket_key.as_slice());
                match stream.write(return_header.to_string().as_bytes()) {
                    Ok(result) => {
                        println!("Adding new connection to pool...")
                        let socket = Socket::new(return_header.sec_websocket_accept.as_slice(), stream);
                        let action = Action::new("new_connection", socket);
                        to_conn_pool.send(action);
                    }
                    Err(e) => {
                        println!("Error writing to stream: {}", e)
                    }
                }
            } else {
                println!("Request header not valid");
                // TODO - return a rejected header
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
fn event_loop(mut server: Server, from_server: Receiver<Action>, to_event_loop: Sender<Action>) {
    let mut sockets: Vec<Socket> = Vec::new();
    let mut socket_msgers: Vec<SocketMessenger> = Vec::new();
    
    loop {
        match from_server.try_recv() {
            Ok(action) => {
                match action.event.as_slice() {
                    "new_connection" => {
                        let mut socket = action.socket.clone();
                        let (to_socket, from_event_loop): (Sender<Message>, Receiver<Message>) = channel();
                        let sock_msgr = SocketMessenger {
                            id: socket.id.clone(),
                            to_socket: to_socket.clone()   
                        };
                        socket_msgers.push(sock_msgr);
                        let mut socket_server = server.clone();
                        socket_server.sockets.push(socket.clone());
                        socket_server.to_event_loop = to_event_loop.clone();
                        spawn(proc() {
                            start_new_socket(socket, from_event_loop, socket_server)
                        });
                    }
                    "drop_connection" => {

                    }
                    "broadcast" => {

                    }
                    "send" => {
                        // Find the handler to this socket's out stream
                        for item in socket_msgers.iter() {
                            if item.id == action.socket.id {
                                item.to_socket.send(action.message.clone())
                            }
                        }
                    }
                    _ => {

                    }
                }
            }
            Err(e) => {
                // TODO - Maybe some stuff?
            }
        }
    }
}

/*
 * Starts the I/O process for a new socket connection
 */
fn start_new_socket(socket: Socket, from_event_loop: Receiver<Message>, mut server: Server) {
    // Start this socket's write stream in another process
    let mut out_stream = socket.stream.clone();
    spawn(proc() {
        loop {
            let msg = from_event_loop.recv();
            msg.send(&mut out_stream).unwrap();
        }
    });

    // Open up a read from this socket and wait till something is received
    loop {
        let mut in_stream = socket.stream.clone();
        let msg = Message::load(&mut in_stream).unwrap();
        match msg.payload {
            Text(ptr) => {
                let json_slice = (*ptr).as_slice();
                server.socket_id = socket.id.clone();
                println!("Socket: {} recevied: {}", socket.id, json_slice);
                determine_event(json_slice, server.clone());
            }
            Binary(ptr) => {
                // TODO - Do awesome binary shit
            }
        }
    }    
}

/*
 * Decodes received JSON
 * Expects the JSON to be in the following format:
 *     {
 *         "event": "EVENT_NAME",
 *         "date": {
 *             // Important data stuff
 *         }
 *     }
 */
fn determine_event(json_data: &str, server: Server) {
    match json::from_str(json_data) {
        Ok(result) => {
            println!("JSON decoded as: {}", result)
            let json_object = result.as_object().unwrap();
            let req_event = json_object.find(&String::from_str("event")).unwrap().as_string().unwrap();
            let passed_data = json_object.find(&String::from_str("data")).unwrap();

            for event in server.events.iter() {
                if req_event == event.name.as_slice() {
                    let callback = event.execute;
                    callback(passed_data.clone(), server.clone());
                }
            }
        }
        Err(e) => {
            println!("Error deserializing json: {}", e)
        }
    }
}

