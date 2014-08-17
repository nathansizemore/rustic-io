// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


extern crate rust_crypto = "rust-crypto";
extern crate serialize;

use std::str;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use self::rust_crypto::digest::Digest;
use self::rust_crypto::sha1::Sha1;
use self::serialize::base64::{ToBase64, STANDARD};
use self::server::Server;
use self::action::Action;
use self::server::socket::Socket;
use self::server::event::Event;
use self::message::Message;
use self::message::{Message, TextOp, Text, BinaryOp, Binary};


mod action;
mod message;
pub mod server;


pub fn start(server: Server, ip: &str, port: u16) {

    /*
     * Communication channels
     *     - From HTTP Server to Connection Pool (Action Passed)
     *     - From Connection Pool to Event Loop (Vec<Sockets> Passed)
     */
    let (to_conn_pool, from_server): (Sender<Action>, Receiver<Action>) = channel();
    let (to_event_loop, from_conn_pool): (Sender<Vec<Socket>>, Receiver<Vec<Socket>>) = channel();

    // Connection Pool Task
    spawn(proc() {
        connection_pool(from_server, to_event_loop)
    });

    // Event Loop Task
    spawn(proc() {
        event_loop(server.events.clone(), from_conn_pool)
    });

    // TCP Server
    let listener = TcpListener::bind(ip, port);
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {
                println!("Error: {}", e)

                // TODO - Handle errors and shit
            }
            Ok(stream) => {
                let to_conn_pool_clone = to_conn_pool.clone();
                spawn(proc() {                
                    process_new_connection(stream, to_conn_pool_clone)
                })
            }
        }
    }

    // Close resources when loop stops/fails or some shit I need to figure out
    drop(acceptor);
}

/*
 * Connection Pool
 *     - Maintains all connections
 *     - On connect/disconnect - issues out new socket array to event loop
 */
fn connection_pool(from_server: Receiver<Action>, to_event_loop: Sender<Vec<Socket>>) {
    let mut connection_list: Vec<Socket> = Vec::new();
    loop {
        let new_connection = from_server.recv();
        match new_connection.event.as_slice() {
            "new_connection" => {
                // Add socket to connection list
                connection_list.push(new_connection.socket);

                // Send new list to event loop
                to_event_loop.send(connection_list.clone());
            }
            "drop_connection" => {
                // TODO - Remove from connection_list
                // TODO - Do some other shit?
            }
            _ => {
                // TODO - Default handler here probably isnt the best
                println!("Uhm, so... I got some shit?")
            }
        }
    }
}

/*
 * Event Loop
 *     - Listens for new socket array generated from Connection Pool
 *     - Cycles through socket array's stream checking for data
 */
fn event_loop(events: Vec<Event>, from_conn_pool: Receiver<Vec<Socket>>) {
    let mut sockets: Vec<Socket> = Vec::new();
    loop {
        // New connection?
        match from_conn_pool.try_recv() {
            Ok(socket_list) => {
                sockets = socket_list;
            }
            Err(e) => {
                // println!("Error: {}", e)
                // TODO - Handle errors and shit
            }
        }

        // Event recevied?
        for socket in sockets.iter() {
            let mut stream = socket.stream.clone();
            let msg = Message::load(&mut stream).unwrap();
            let (payload, opcode) = match msg.payload {
                Text(p) => {
                    println!("Received: {}", (*p).as_slice());
                    (Text(box String::from_str("Received: ").append((*p).as_slice())), TextOp)
                }
                Binary(p) => {
                    (Binary(p), BinaryOp)
                }
            };
        }
    }
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
            for line in header.split_str("\r\n") {
                let key_value: Vec<&str> = line.split(' ').collect();
                if key_value[0] == "Sec-WebSocket-Key:" {
                    return_accept_header(stream.clone(), key_value[1], to_conn_pool.clone())
                }
            }
        }
        None => {
            println!("Buffer not valid UTF-8")
        }
    }
}

/*
 * Returns the WebSocket Protocol Accept Header to the requesting client
 *
 * Accept Header:
 * HTTP/.1 101 Switching Protocols
 * Upgrade: websocket
 * Connection: Upgrade
 * Sec-WebSocket-Accept: COMPUTED_VALUE
 *
 * Steps to create the Sec-WebSocket-Accept Key:
 * 1.) Append "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" to passed key
 * 2.) SHA-1 Hash that value
 * 3.) Base64 encode the hashed bytes, not string
 * 4.) Return Base64 encoded bytes, not string
 */ 
fn return_accept_header(mut stream: TcpStream, key: &str, to_conn_pool: Sender<Action>) {
    // Combine key and WebSocket Key API thing        
    let mut pre_hash = String::from_str(key);
    pre_hash.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    //Get the SHA-1 Hash as bytes
    let mut out = [0u8, ..20];
    sha1_hash(pre_hash.as_slice(), out);

    //Base64 encode the buffer
    let mut config = STANDARD;
    let mut encoded = out.to_base64(config);

    //Build the accept header
    let mut accept_header = String::from_str("HTTP/1.1 101 Switching Protocols\r\n");
    accept_header.push_str("Upgrade: websocket\r\n");
    accept_header.push_str("Connection: Upgrade\r\n");
    accept_header.push_str("Sec-WebSocket-Accept: ");
    let mut based_bytes;
    unsafe {
        based_bytes = encoded.as_bytes();
        accept_header.push_bytes(based_bytes);
    }
    accept_header.push_str("\r\n\r\n");

    //Return header to asking client
    match stream.write(accept_header.as_bytes()) {
        Ok(result) => {
            println!("Adding new connection to pool...")
            let socket = Socket::new(encoded.as_slice(), stream);
            let action = Action::new("new_connection", socket);
            to_conn_pool.send(action);
        }
        Err(e) => {
            println!("Error writing to stream: {}", e)
        }
    }
}

/*
 * SHA-1 Hash performed on passed value
 * Bytes placed in passed out buffer
 */
fn sha1_hash(value: &str, out: &mut [u8]) {
    let mut sha = box Sha1::new();
    (*sha).input_str(value);
    sha.result(out);
}




















