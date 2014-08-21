// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.


use std::io::{TcpStream};

/*
 * Struct representing a websocket
 * id is the returned Sec-Socket-Key in return header
 */
pub struct Socket<'a> {
	pub id: String,
	pub stream: TcpStream
}

impl<'a> Socket<'a> {

	// Constructs a Socket object
	pub fn new(id: &str, stream: TcpStream) -> Socket<'a> {
		Socket {
			id: String::from_str(id),
			stream: stream
		}
	}
}

impl<'a> Clone for Socket<'a> {
	fn clone(&self) -> Socket<'a> {
		Socket {
			id: self.id.clone(),
			stream: self.stream.clone()
		}
	}
}