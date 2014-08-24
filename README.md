rustic-io [<img src="https://travis-ci.org/nathansizemore/rustic-io.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io)
=========

rustic-io is a simple websocket server library written in Rust, inspired by socket.io.  It aims to be a fast, concurrent websocket server library for text and binary messages.

Borrows messaging implementation from [rust-ws](https://github.com/ehsanul/rust-ws)

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
  * Figure out better way to parse and handle errors with JSON
  * Create default module for rustic-io event JSONs
* **Binary Messages**
  * Implement the shit
* **JavaScript Library**
  * Create a client side JavaScript library for abstracting rustic-io communication

#### Example Usage
```rust
fn main() {
    let mut server = Server::new();
    server.on("hello", echo_back);
    rustic_io::start(server, "127.0.0.1", 1338);
}

fn echo_back(data: json::Json, server: Server) {
    let json_object = data.as_object().unwrap();
    let msg = json_object.find(&String::from_str("msg")).unwrap().as_string().unwrap();
    let (payload, mask) = (Text(box String::from_str(msg)), TextOp);
    server.send(Message {
        payload: payload,
        mask: mask
    });
}
```

#### Example Projects
* [Echo Server](https://github.com/nathansizemore/rustic-io-echo-server) [<img src="https://travis-ci.org/nathansizemore/rustic-io-echo-server.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io-echo-server)
  