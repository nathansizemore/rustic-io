// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.

use self::rustic_io::server::Server;

#[path="lib/rustic_io.rs"]
mod rustic_io;

fn main() {

    //Get yo'self a server
    let mut server = Server::new();

    // Register events
    server.on("connection", on_connection);
    server.on("hello", on_hello);

    // Start server
    rustic_io::start(server, "127.0.0.1", 1338);
}

fn on_connection(data: &str, server: Server) {
    println!("on_connection called...");
}

fn on_hello(data: &str, server: Server) {
    println!("on_hello called...");
}





// pub fn new<'a>() -> Server<'a> {
//     Server::new()
// }


// pub fn start(server: Server, ip: &str, port: u16) {

//     // Start connection pool
//     let (to_conn_pool, from_server): (Sender<Action>, Receiver<Action>) = channel();
//     spawn(proc() {
//         connection_pool(from_server)
//     });

//     // Start event loop
//     let (to_event_loop, from_server): (Sender<Socket>, Receiver<Socket>) = channel();

//     // Start TCP Server
//     let listener = TcpListener::bind(ip, port);
//     let mut acceptor = listener.listen();

//     for stream in acceptor.incoming() {
//         match stream {
//             Err(e) => {
//                 println!("Error: {}", e)
//                 // TODO - Handle errors and shit
//             }
//             Ok(stream) => {
//                 let to_conn_pool_clone = to_conn_pool.clone();
//                 spawn(proc() {                
//                     process_new_connection(stream, to_conn_pool_clone)
//                 })
//             }
//         }
//     }

//     //Close when loop fails (Should be never)
//     drop(acceptor);
// }

// /*
//  * Accepts and parses input stream from a new connection
//  * looking for the Sec-WebSocket-Key header in the HTTP/1.1 Request
//  *
//  * Executed on separate thread.  Exits if the header is not found
//  */
// fn process_new_connection(mut stream: TcpStream, to_conn_pool: Sender<Action>) {
//     let mut buffer = [0, ..1024]; // 512 may be fine here?
//     match stream.read(buffer) {
//         Ok(result) => {
//             println!("Ok: {}", result)
//         }
//         Err(e) => {
//             println!("Error: {}", e)
//             // TODO - Handle errors and shit
//         }
//     }
    
//     //Parse request for Sec-WebSocket-Key
//     match str::from_utf8(buffer) {
//         Some(header) => {
//             println!("{}", header);
//             for line in header.split_str("\r\n") {
//                 let key_value: Vec<&str> = line.split(' ').collect();
//                 if key_value[0] == "Sec-WebSocket-Key:" {
//                     return_accept_header(stream.clone(), key_value[1], to_conn_pool.clone())
//                 }
//             }
//         }
//         None => {
//             println!("Buffer not valid UTF-8")
//         }
//     }
// }


//  * Returns the WebSocket Protocol Accept Header to the requesting client
//  *
//  * Accept Header:
//  * HTTP/.1 101 Switching Protocols
//  * Upgrade: websocket
//  * Connection: Upgrade
//  * Sec-WebSocket-Accept: COMPUTED_VALUE
//  *
//  * Steps to create the Sec-WebSocket-Accept Key:
//  * 1.) Append "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" to passed key
//  * 2.) SHA-1 Hash that value
//  * 3.) Base64 encode the hashed bytes, not string
//  * 4.) Return Base64 encoded bytes, not string
 
// fn return_accept_header(mut stream: TcpStream, key: &str, to_conn_pool: Sender<Action>) {
//     // Combine key and WebSocket Key API thing        
//     let mut pre_hash = String::from_str(key);
//     pre_hash.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

//     //Get the SHA-1 Hash as bytes
//     let mut out = [0u8, ..20];
//     sha1_hash(pre_hash.as_slice(), out);

//     //Base64 encode the buffer
//     let mut config = STANDARD;
//     let mut encoded = out.to_base64(config);

//     //Build the accept header
//     let mut accept_header = String::from_str("HTTP/1.1 101 Switching Protocols\r\n");
//     accept_header.push_str("Upgrade: websocket\r\n");
//     accept_header.push_str("Connection: Upgrade\r\n");
//     accept_header.push_str("Sec-WebSocket-Accept: ");
//     let mut based_bytes;
//     unsafe {
//         based_bytes = encoded.as_bytes();
//         accept_header.push_bytes(based_bytes);
//     }
//     accept_header.push_str("\r\n\r\n");

//     //Return header to asking client
//     match stream.write(accept_header.as_bytes()) {
//         Ok(result) => {
//             println!("Adding new connection to pool...")
//             let socket = Socket::new(encoded.as_slice(), stream);
//             let action = Action::new("new_connection", socket);
//             to_conn_pool.send(action);
//         }
//         Err(e) => {
//             println!("Error writing to stream: {}", e)
//         }
//     }
// }

// /*
//  * SHA-1 Hash performed on passed value
//  * Bytes placed in passed out buffer
//  */
// fn sha1_hash(value: &str, out: &mut [u8]) {
//     let mut sha = box Sha1::new();
//     (*sha).input_str(value);
//     sha.result(out);
// }

// fn connection_pool(receiver: Receiver<Action>) {
//     let mut sockets: Vec<Socket> = Vec::new();
//     loop {
//         let action = receiver.recv();
//         match action.event.as_slice() {
//             "new_connection" => {

//                 // Add to connection pool
//                 println!("Oh snap, new connection in the pool!")
//                 sockets.push(action.socket)

//                 // Pass socket into event loop
                

//             }
//             _ => {
//                 println!("Default option hit in connection_pool")
//             }
//         }
//     }
// }

// fn event_loop(socket_receiver: Receiver<Socket>) {
//     let mut sockets: Vec<Socket> = Vec::new();
//     loop {
//         //let new_socket = socket_receiver.
//     }
// }

























