// The MIT License (MIT)

// Copyright (c) 2014 Nathan Sizemore

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.


extern crate rust_crypto = "rust-crypto";
extern crate serialize;

use std::str;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use self::rust_crypto::digest::Digest;
use self::rust_crypto::sha1::Sha1;
use self::serialize::base64::{ToBase64, STANDARD};
use self::server::Server;
//use self::action::Action;

//mod action;
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
                    process_new_connection(stream, to_conn_pool.clone())
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
    let mut connection_list: Vec<Socket>> = Vec::new();
    loop {
        // Do all the things forever
    }
}



/*
 * Event Loop
 *     - Listens for new socket array generated from Connection Pool
 *     - Cycles through socket array's stream checking for data
 */
fn event_loop(server: Server, from_conn_pool: Receiver<Vec<Socket>>) {

}



















