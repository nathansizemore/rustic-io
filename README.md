rustic-io [<img src="https://travis-ci.org/nathansizemore/rustic-io.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io)
=========

rustic-io is a simple websocket server library written in Rust, inspired by socket.io.  It aims to be a fast, concurrent websocket server library for text and binary messages.

Borrows messaging implementation from [rust-ws](https://github.com/ehsanul/rust-ws)

Pull requests are welcomed, and encouraged; because I have no idea what I am doing.  I just started banging on the keyboard and this is what I ended up with.

#### Current State
Incomplete. JavaScript library that abstracts websockets is now available.

#### TODOs
* **Crate It Up Correctly?**
  * Work on the Cargo files, naming, organization to make this a more Rust-styled crate
    * It works now, because I just randonly move shit until it compiles :)  But would like to get a better handle on the cargo files and versioning and stuffs
* **HTTP Header**
  * Implement better parsing for incoming HTTP header
    * Right now, all it cares about is the Sec-WebSocket-Key field
* **JSON Messages**
  * Figure out better way to parse and handle errors with JSON
  * Create default module for rustic-io event JSONs
* **Binary Messages**
  * Implement the shit
* **Disconnects/Closing Connections**
  * Needs implemented

#### Example Usage

##### On the Server
```rust
#[deriving(Decodable, Encodable)]
pub struct Foo {
    msg: String
}

fn main() {
    let mut server = Server::new();
    server.on("hello", echo_back);
    rustic_io::start(server, "127.0.0.1", 1338);
}

fn echo_back(data: json::Json, server: Server) {
    let json_object = data.as_object().unwrap();
    let msg = json_object.find(&String::from_str("msg")).unwrap().as_string().unwrap();
    server.send("echo", json::encode(&Foo {
        msg: String::from_str(msg)
    }));
}
```
##### In the Browser
```javascript
function main() {
    // Create a socket
    var socket = new rustic_io('127.0.0.1', 1338);

    // Listen for events
    socket.on('echo', function(socket, data) {
        // Do some shit with data

        // Want to send the server something else?
        socket.send('something else', {
            something: 'else'
        });
    });

    // Send events
    socket.send('hello', {
        msg: 'What up, Rust?'
    });
}
```

#### Example Projects
* [Echo Server](https://github.com/nathansizemore/rustic-io-echo-server) [<img src="https://travis-ci.org/nathansizemore/rustic-io-echo-server.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io-echo-server)
  