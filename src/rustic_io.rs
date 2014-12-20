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
extern crate "crypto" as rust_crypto;

use std::str;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};

use self::server::Server;
use self::action::Action;
use self::httpheader::{RequestHeader, ReturnHeader};

pub mod action;
pub mod event;
pub mod eventloop;
pub mod httpheader;
pub mod message;
pub mod server;
pub mod socket;


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
    let (action_sender, action_receiver): (Sender<Action>, Receiver<Action>) = channel();
    let (new_conn_sender, new_conn_receiver): (Sender<TcpStream>, Receiver<TcpStream>) = channel();
    let event_list = server.events.clone();
    
    spawn(move || {
        eventloop::start(action_sender, action_receiver, new_conn_receiver, event_list)
    });

    // Start TCP Server
    let mut address = String::new();
    address.push_str(server.ip.as_slice());
    address.push_str(":");
    address.push_str(server.port.as_slice());

    let listener = TcpListener::bind(address.as_slice());
    let mut acceptor = listener.listen();
    for stream in acceptor.incoming() {
        match stream {
            Ok(stream) => {
                let new_conn_sender_clone = new_conn_sender.clone();
                spawn(move || {
                    process_new_tcp_connection(stream, new_conn_sender_clone)
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
            if request_header.is_valid() {
                let return_header = ReturnHeader::new(request_header.sec_websocket_key.as_slice());
                match stream.write(return_header.to_string().as_bytes()) {
                    Ok(result) => {
                        new_conn_sender.send(stream.clone());
                    }
                    Err(e) => { /* User closed connection */ }
                }
            }
        }
        None => { /* Don't care */ }
    }
}

