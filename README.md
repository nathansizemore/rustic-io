rustic-io [<img src="https://travis-ci.org/nathansizemore/rustic-io.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io)
=========

rustic-io is a simple websocket server library written in Rust.  It aims to be an easy to implement, fast, and concurrent websocket server library for event based messaging.

#### Example Usage
```rust
#[deriving(Decodable, Encodable)]
pub struct Foo {
    msg: String
}

fn main() {
    let mut server = rustic_io::new_server("127.0.0.1", "1338");
    server.on("some_event", function_to_execute);
    rustic_io::start(server);
}

fn function_to_execute(data: json::Json, socket: Socket) {
    let json_object = data.as_object().unwrap();

    // Do some stuff with received data...

    // Create some object to send back
    let bar = Foo {
        msg: String::from_str("Hello from Rust!")
    };

    // Send some event back to socket
    socket.send("some_event", json::encode(&bar));

    // Or, broadcast that event to all sockets
    socket.broadcast("some_event", json::encode(&bar));
}
```

#### Example Projects
* [Echo Server](https://github.com/nathansizemore/rustic-io-echo-server) [<img src="https://travis-ci.org/nathansizemore/rustic-io-echo-server.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io-echo-server)

#### Credits
WebSocket payload implementation from [rust-ws](https://github.com/ehsanul/rust-ws)
