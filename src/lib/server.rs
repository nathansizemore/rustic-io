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


/*
 * Structure holding server information, sockets 
 * and pointer to receving event loop
 */
pub struct Server {
    ip: String,
    port: u16,
    //sockets: Vec<Socket>,
    event_loop: fn()
}


impl Server {

    // Constructs a Server object
    pub fn new(ip_addr: String, port_num: u16, event_loop: fn()) -> Server {
        Server {
            ip: ip_addr,
            port: port_num,
            event_loop: event_loop
        }
    }

    /*
     * Binds to specified ip and port
     * Creates a new process (thread) for 
     * each new connection request
     */
    //pub fn on_connection(&self, event_loop: fn()) {
        //self.event_loop = event_loop;
        /*
        let listener = TcpListener::bind(self.ip.as_slice(), self.port);
        let mut acceptor = listener.listen();

        for stream in acceptor.incoming() {
            match stream {
                Err(e) => {
                    println!("Error: {}", e)
                    // TODO - Handle errors and shit
                }
                Ok(stream) => spawn(proc() {
                    process_new_connection(stream)
                })
            }
        }

        drop(acceptor);
        */
    //}

    /*
     * Accepts and parses input stream from a new connection
     * looking for the Sec-WebSocket-Key header in the HTTP/1.1 Request
     *
     * Executed on separate thread.  Exits if the header is not found
     */
    fn process_new_connection(&self, mut stream: TcpStream) {
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
                        self.return_accept_header(stream.clone(), key_value[1])
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
    fn return_accept_header(&self, stream: TcpStream, key: &str) {

    }

    fn sha1_hash(value: &str) -> [u8, ..20] {
        let mut out = [0u8, ..20];
        let mut sha = box Sha1::new();
        (*sha).input_str(value);
        sha.result(out);
        return out;
    }
}


























