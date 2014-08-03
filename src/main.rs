

extern crate rust_crypto = "rust-crypto";
extern crate serialize;

use std::str;
use std::string::String;
use std::io::{TcpListener, TcpStream};
use std::io::{Listener, Acceptor};
use rust_crypto::digest::Digest;
use rust_crypto::sha1::Sha1;
use serialize::base64::{ToBase64, STANDARD};




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
    match stream.read(buffer) {
        Ok(result) => {
            println!("OK: {}", result)
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
    //stream.close_read();
    
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

    //Add websocket api key
    let mut pre_hash = String::from_str(key);
    pre_hash.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

    //Hash the string with SHA-1
    let mut out = [0u8, ..20];
    let mut sha = box Sha1::new();
    let mut s = pre_hash.as_slice();
    (*sha).input_str(s);
    sha.result(out);

    //Base64 encode the hash
    let mut config = STANDARD;
    let mut based2 = out.to_base64(config);

    //Build response header with accept key
    let mut accept_header = String::from_str("HTTP/1.1 101 Switching Protocols\r\n");
    accept_header.push_str("Upgrade: websocket\r\n");
    accept_header.push_str("Connection: Upgrade\r\n");
    accept_header.push_str("Sec-WebSocket-Accept: ");
    unsafe
    {
        accept_header.push_bytes(based2.as_mut_bytes());
    }
    accept_header.push_str("\r\n");
    //accept_header.push_str("Sec-WebSocket-Protocol: chat\r\n");
    accept_header.push_str("\r\n");
    println!("{}", accept_header);    


    //Echo back accept response
    match stream.write(accept_header.as_bytes()) {
        Ok(result) => {
            println!("stream write ok: {}", result)
        }
        Err(e) => {
            println!("Error on stream.write: {}", e)
        }
    }
    //stream.close_write();
}




