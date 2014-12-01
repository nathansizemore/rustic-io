// Copyright (c) 2014 Nathan Sizemore

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.


use super::serialize::json::Json;

use super::socket::Socket;

/*
 * Struct representing an event, and a function to execute
 * when that event is received from the client
 */
pub struct Event {
    pub name: String,
    pub execute: fn(data: Json, server: Socket)
}

impl Event {

    // Constructs an Event object
    pub fn new(event: &str, execute: fn(data: Json, server: Socket)) -> Event {
        Event {
            name: String::from_str(event),
            execute: execute
        }
    }
}

impl Clone for Event {
    fn clone(&self) -> Event {
        Event {
            name: self.name.clone(),
            execute: self.execute
        }
    }
}
