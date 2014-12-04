rustic-io [<img src="https://travis-ci.org/nathansizemore/rustic-io.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io)
=========

rustic-io is a simple websocket server library written in Rust.  It aims to be an easy to implement, fast, and concurrent websocket server library for text and binary messages.

#### TODOs
* **Binary Messages**
  * Needs implemented

#### Example Usage
```rust
#[deriving(Decodable, Encodable)]
pub struct Foo {
    msg: String
}

fn main() {
    let mut server = rustic_io::new_server("127.0.0.1", "1338");
    server.on("tell_just_me", tell_just_me);
    server.on("tell_erry_body", tell_erry_body);
    rustic_io::start(server);
}

fn tell_just_me(data: json::Json, socket: Socket) {
  let json_object = data.as_object().unwrap();
  let msg = json_object.get(&String::from_str("msg")).unwrap().as_string().unwrap();
    socket.send("echo", json::encode(&Foo {
        msg: String::from_str(msg)
    }));
}

fn tell_erry_body(data: json::Json, socket: Socket) {
    let json_object = data.as_object().unwrap();
    let msg = json_object.get(&String::from_str("msg")).unwrap().as_string().unwrap();
    socket.broadcast("echo", json::encode(&Foo {
        msg: String::from_str(msg)
    }));
}
```

#### Example Projects
* [Echo Server](https://github.com/nathansizemore/rustic-io-echo-server) [<img src="https://travis-ci.org/nathansizemore/rustic-io-echo-server.png?branch=master">](https://travis-ci.org/nathansizemore/rustic-io-echo-server)

#### Credits
WebSocket payload implementation from [rust-ws](https://github.com/ehsanul/rust-ws)
  