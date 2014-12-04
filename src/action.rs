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


use super::message::Message;
use super::message::Payload::{Text, Binary};
use super::message::Mask::{TextOp, BinaryOp};


/*
 * Struct representing an Action the Event Loop needs to execute
 */
pub struct Action {
    pub event: String,
    pub message: Message,
    pub socket_id: String
}

impl Action {

    // Constructs a new action
    pub fn new(event: &str) -> Action {
        // Build a default message for when action does not need a message
        let (payload, mask) = (Text(box String::from_str("blah, blah")), TextOp);
        let msg = Message {
            payload: payload,
            mask: mask
        };

        Action {
            event: String::from_str(event),
            message: msg.clone(),
            socket_id: String::from_str("")
        }
    }
}
