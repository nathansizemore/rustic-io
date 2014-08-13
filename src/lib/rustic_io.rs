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

mod action;
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
                spawn(proc() {                
                    //process_new_connection(stream, to_conn_pool.clone())
                })
            }
        }
    }

    //Close when loop fails (Should be never)
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
        // from_server read (Blocking)
        
        // Match the result
        // - new_connection -> add socket to connection_list
        //      - send new list to event loop
    }
}

/*
 * Event Loop
 *     - Listens for new socket array generated from Connection Pool
 *     - Cycles through socket array's stream checking for data
 */
fn event_loop(events: Vec<Event>, from_conn_pool: Receiver<Vec<Socket>>) {
    
}



















