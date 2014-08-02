

extern crate rust_crypto = "rust-crypto";
extern crate serialize;

use std::str;
use std::string::String;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use rust_crypto::digest::Digest;
use rust_crypto::sha1::Sha1;
use serialize::base64::{Config, ToBase64};




//Entry point
fn main() {
    //Setup socket
    let listener = TcpListener::bind("127.0.0.1", 1338);
    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => {
                //Handle errors
            }
            Ok(stream) => spawn(proc() {
                process_incoming_connection(stream)
            })
        }
    }

    drop(acceptor);
}

fn process_incoming_connection(mut stream: TcpStream) {

    //Grab the incoming buffer
    let mut buffer = [0, ..1024];
    stream.read(buffer);
    
    //Parse request for Sec-WebSocket-Key
    match str::from_utf8(buffer) {
        Some(header) => {
            println!("{}", header);
            for line in header.split_str("\r\n") {
                let key_value: Vec<&str> = line.split(' ').collect();
                if key_value[0] == "Sec-WebSocket-Key:" {
                    accept_incoming_connection(stream.clone(), key_value[1])
                }
            }
        }
        None => {
            println!("Buffer not valid UTF-8")
        }
    }
}

fn accept_incoming_connection(mut stream: TcpStream, key: &str) {
    println!("Key found!");
    println!("{}", key);

    //Add websocket api key
    let mut pre_hash = String::from_str(key);
    pre_hash.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    //Hash the string with SHA-1
    let mut out = [0u8, ..20];
    let mut sha = box Sha1::new();
    let mut s = pre_hash.as_slice();
    (*sha).input_str(s);
    sha.result(out);
    let hashed_value = (*sha).result_str();

    //Base64 encode the hash


    //Build response header with accept key


    //Echo back accept response
}



