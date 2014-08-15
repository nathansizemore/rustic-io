// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.

/*
 * Example program showing use case for rustic_io websocket lib
 */

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


