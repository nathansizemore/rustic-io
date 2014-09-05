// Copyright (c) 2014 Ehsanul Hoque

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


use std::io::{IoResult, IoError, TcpStream};
use std::num;

/*
 * Enum representing the data and type send with message
 */
#[deriving(Clone)]
pub enum Payload {
    Text(Box<String>),
    Binary(Vec<u8>),
    Empty
}

/*
 * Enum representing the various mask values for Websocets
 */
#[deriving(FromPrimitive)]
#[deriving(Clone)]
pub enum Mask {
    ContinuationOp = 0x0,
    TextOp = 0x1,
    BinaryOp = 0x2,
    CloseOp = 0x8
}

/*
 * Struct representing data received/sent to/from websocket
 */
pub struct Message {
    pub payload: Payload,
    pub mask: Mask
}

impl Message {

    /*
     * Reads from the passed stream
     * When data received, determines what type and returns the result
     * of the operation
     */
    pub fn load(stream: &mut TcpStream) -> IoResult<Box<Message>> {
        match stream.read_exact(2) {
            Ok(vec1) => {
                let buf1 = vec1.as_slice();

                // Get the mask to determine type of data being sent
                let mask = buf1[0] & 0b0000_1111;
                let mask: Mask = num::from_u8(mask).unwrap();        
                let pay_len = buf1[1] & 0b0111_1111;

                // Determine length of the payload
                let payload_length = match pay_len {
                    127 => try!(stream.read_be_u64()), // 8 bytes in network byte order
                    126 => try!(stream.read_be_u16()) as u64, // 2 bytes in network byte order
                    _   => pay_len as u64
                };

                // Grab the payload information
                match stream.read_exact(4) {
                    Ok(masking_key_vec) => {
                        // Grab the payload
                        let masking_key_buf = masking_key_vec.as_slice();
                        match stream.read_exact(payload_length as uint) {
                            Ok(masked_payload_buf) => {
                                let mut payload_buf = vec!();
                                for (i, &octet) in masked_payload_buf.iter().enumerate() {
                                    payload_buf.push(octet ^ masking_key_buf[i % 4]);
                                }

                                // Build specific payload based on mask type
                                let payload: Payload = match mask {
                                    TextOp => {
                                        Text(box String::from_utf8(payload_buf).unwrap())
                                    }
                                    BinaryOp => {
                                        Binary(payload_buf)
                                    }
                                    CloseOp => {
                                        Empty
                                    }
                                    _ => {
                                        unimplemented!()
                                    }
                                };

                                // Build result to return
                                let message = box Message {
                                    payload: payload,
                                    mask: mask
                                };

                                return Ok(message);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                return Err(e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Err(e);
            }
        }
    }

    /*
     * Writes out message to the passed stream
     */
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {

        // Grab the length of the data being sent
        let payload_length = match self.payload {
            Text(ref p) => p.len(),
            Binary(ref p) => p.len(),
            Empty => 0
        };

        // Write out the type of data
        try!(stream.write_u8(0b1000_0000 | self.mask as u8));

        // Write out the length of the data
        if payload_length <= 125 {
            try!(stream.write_u8(payload_length as u8));
        } else if payload_length <= 65535 {
            try!(stream.write_u8(126));
            try!(stream.write_be_u16(payload_length as u16));
        } else if payload_length > 65535 {
            try!(stream.write_u8(127));
            try!(stream.write_be_u64(payload_length as u64));
        }

        // Write out the data
        match self.payload {
            Text(ref p) => {
                try!(stream.write((*p).as_slice().as_bytes()))
            }
            Binary(ref p) => {
                try!(stream.write((*p).as_slice()))
            }
            Empty => {
                ()
            }
        }

        // Reset the stream
        try!(stream.flush());

        // TODO - handle Err shit
        
        return Ok(());
    }
}

impl Clone for Message {
    fn clone(&self) -> Message {
        Message {
            payload: self.payload.clone(),
            mask: self.mask.clone()
        }
    }
}

