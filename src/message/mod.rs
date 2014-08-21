// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// Version 2, December 2004

// Copyright (C) 2014 Nathan Sizemore <nathanrsizemore@gmail.com>

// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.

// DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
// TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION

// 0. You just DO WHAT THE FUCK YOU WANT TO.

use std::io::net::tcp::TcpStream;
use std::io::IoResult;
use std::num;

/*
 * Enum representing the data and type send with message
 */
#[deriving(Clone)]
pub enum Payload {
    Text(Box<String>),
    Binary(Vec<u8>)
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
        let vec1 = try!(stream.read_exact(2));
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

        // Grab the data from the stream
        let masking_key_vec = try!(stream.read_exact(4));
        let masking_key_buf = masking_key_vec.as_slice();
        let masked_payload_buf = try!(stream.read_exact(payload_length as uint));

        // Grab the payload from the buffer
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

            // TODO - Implement continuation and close shit
            
            _ => {
                unimplemented!()
            }            
        };

        // Build result to return
        let message = box Message {
            payload: payload,
            mask: mask
        };

        // TODO - Implement Err shit

        return Ok(message);
    }

    /*
     * Writes out message to the passed stream
     */
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {

        // Grab the length of the data being sent
        let payload_length = match self.payload {
            Text(ref p) => p.len(),
            Binary(ref p) => p.len(),
        };

        // Write out the type of data
        try!(stream.write_u8(0b1000_0000 | self.mask as u8));

        // Write out the length of the data
        if payload_length <= 125 {
            try!(stream.write_u8(payload_length as u8));
        } else if payload_length <= 65536 {
            try!(stream.write_u8(126));
            try!(stream.write_be_u16(payload_length as u16));
        } else if payload_length > 65536 {
            try!(stream.write_u8(127));
            try!(stream.write_be_u64(payload_length as u64));
        }

        // Write out the data
        match self.payload {
            Text(ref p)   => try!(stream.write((*p).as_slice().as_bytes())),
            Binary(ref p) => try!(stream.write((*p).as_slice())),
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

