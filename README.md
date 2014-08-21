rustic-io
=========

rustic-io is a simple websocket server library written in Rust.  It's aim is to provide a much faster and scalable server side websocket library than possible with Node.js.

###### Current State
Incomplete.

###### Current State Abilities
* Text only messages in a specific JSON format

###### Current State Process
* Recevies an HTTP Websocket upgrade request, parse the header looking for Sec-Websocket-Key
* If found, returns HTTP Accept header, if not found, does nothing
* Creates new tasks for that socket's I/O
* Upon receiving JSON, looks for the "event" key, grabs it's value and executes the function associated with that event name

###### TODOs
* **HTTP Header Response**
  * Create separate module
  * Cover majority of request/receive headers in that module
* **JSON Messages**
  * Figure out better way to prase and handle errors in JSON
  * Create default module for rustic-io event JSONs
* **Binary Messages**
  * Implement the shit
* **JavaScript Library**
  * Create a Client side JavaScript library for abstracting rustic-io communication

###### Example Usage
```
extern crate rustic_io = "rustic-io";
extern crate serialize;

use std::str;
use serialize::json;
use rustic_io::server::Server;
use rustic_io::message::Message;
use rustic_io::message::{Message, TextOp, Text, BinaryOp, Binary};

fn main() {

    //Get yo'self a server
    let mut server = Server::new();

    // Register events
    server.on("hello", on_hello);

    // Start server
    rustic_io::start(server, "127.0.0.1", 1338);
}

fn on_hello(data: json::Json, server: Server) {
    println!("on_hello called");
    
    // Do some fancy shit with data
    
    // Create a return object
    let thing = Thing {
        event: String::from_str("echo"),
        data: String::from_str("asdf")
    };
    
    // Create the return message
    let (payload, opcode) = (Text(box String::from_str(json::encode(&thing).as_slice())), TextOp);
    let msg = Message {
        payload: payload,
        opcode: opcode
    };

    server.send(msg);


}

#[deriving(Decodable, Encodable)]
pub struct Thing {
    event: String,
    data: String
}
```
  