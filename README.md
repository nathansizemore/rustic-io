rustic-io
=========

rustic-io is a simple websocket server library written in Rust.  It aims to be a fast, scalable websocket server for text and binary messages.

**How it Works**

Sockets are grouped together in an event loop which allows broadcasting to all, and each socket is contained in a separate Rust task, with it's write stream also a separate task.  This allows really fast I/O even under high concurrent loads.
You pass rustic-io the event name you are listening for, and the function you want to execute when that event is received, and it takes care of the rest.

Pull requests are welcomed, and encouraged; because I have no idea what I am doing.  I just started banging on the keyboard and this is what I ended up with.


#### Current State
Incomplete. Only text messages in specific JSON format are supported.

#### TODOs
* **Crate It Up Correctly?**
  * Work on the Cargo files, naming, organization to make this a more Rust-styled crate
    * It works now, because I just randonly move shit until it compiles :)  But would like to get a better handle on the cargo files and versioning and stuffs
* **HTTP Header Response**
  * Implement rejection header
  * Implement better parsing for incoming HTTP header
    * Right now, all it cares about is the Sec-WebSocket-Key field
* **JSON Messages**
  * Figure out better way to prase and handle errors in JSON
  * Create default module for rustic-io event JSONs
* **Binary Messages**
  * Implement the shit
* **JavaScript Library**
  * Create a Client side JavaScript library for abstracting rustic-io communication

#### Example Usage
```
extern crate rustic_io = "rustic-io";
extern crate serialize;

use std::str;
use serialize::json;
use rustic_io::server::Server;
use rustic_io::message::Message;
use rustic_io::message::{Message, TextOp, Text, BinaryOp, Binary};

/*
 * Some fancy thing that gets returned
 */
#[deriving(Decodable, Encodable)]
pub struct Thing {
    event: String,
    data: String
}

fn main() {

    //Get yo'self a server
    let mut server = Server::new();

    // Register events you care about
    server.on("hello", on_hello);

    // Start server
    rustic_io::start(server, "127.0.0.1", 1338);
}

fn on_hello(data: json::Json, server: Server) {
    
    // Do some important shit with data
    
    // Create the fancy return thing
    let thing = Thing {
        event: String::from_str("echo"),
        data: String::from_str("asdf")
    };
    
    // Put that fancy thing into a websocket message
    let (payload, mask) = (Text(box String::from_str(json::encode(&thing).as_slice())), TextOp);
    let msg = Message {
        payload: payload,
        mask: mask
    };

    // Tell the server to send that fancy shit
    server.send(msg);
}
```
  