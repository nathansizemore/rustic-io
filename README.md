rustic-io [<img src="https://travis-ci.org/nathansizemore/rustic-io.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io)
=========

rustic-io is a simple websocket server library written in Rust.  It aims to be an easy to implement, fast, and concurrent websocket server library for event based messaging.

There was thought on binary support to match the protocol, but if that was supported, there would not be a way to parse functionality on event, because the message can either be text or binary.  There is no mixing of the two.  Currently, Vec<T> is supported in JSON ecoding/decoding through Rust, so all "binary" data should be passed as a buffer in your message struct.

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
  