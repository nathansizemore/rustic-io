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

/*
 * Module representing the HTTP Header Request/Response
 * To conform to RFC - 6455 up to the point of allowing
 * Text/Binary messaging.  Covering all websocket connection
 * types is outside the intent of this module.
 *
 * http://tools.ietf.org/html/rfc6455
 */


use super::rust_crypto::digest::Digest;
use super::rust_crypto::sha1::Sha1;
use super::rustc_serialize::base64::{ToBase64, STANDARD};


/*
 * Struct representing a websocket connection request
 */
pub struct RequestHeader {
    pub upgrade: String,
    pub connection: String,
    pub host: String,
    pub origin: String,
    pub pragma: String,
    pub cache_control: String,
    pub sec_websocket_key: String,
    pub sec_websocket_version: String,
    pub sec_websocket_extensions: String,
    pub user_agent: String
}

/*
 * Struct representing the websocket accept header
 */
pub struct ReturnHeader {
    pub heading: String,
    pub upgrade: String,
    pub connection: String,
    pub sec_websocket_accept: String
}

impl RequestHeader {

    // Constructs a new RequestHeader struct
    pub fn new(header: &str) -> RequestHeader {

        // Build a default header
        let mut request_header = RequestHeader {
            upgrade: String::from_str(""),
            connection: String::from_str(""),
            host: String::from_str(""),
            origin: String::from_str(""),
            pragma: String::from_str(""),
            cache_control: String::from_str(""),
            sec_websocket_key: String::from_str(""),
            sec_websocket_version: String::from_str(""),
            sec_websocket_extensions: String::from_str(""),
            user_agent:String::from_str("")
        };

        // TODO - Parse and get the values correctly
        //
        // This is all fucked up because it delimits off spaces
        // All the fields we really care about work with this method, but
        // it would be nice to make it actually find and parse correctly
        for line in header.split_str("\r\n") {
            let key_value: Vec<&str> = line.split(' ').collect();
            match key_value[0] {
                "Upgrade:" => request_header.upgrade = String::from_str(key_value[1]),
                "Connection:" => request_header.connection = String::from_str(key_value[1]),
                "Host:" => request_header.host = String::from_str(key_value[1]),
                "Origin:" => request_header.origin = String::from_str(key_value[1]),
                "Pragma:" => request_header.pragma = String::from_str(key_value[1]),
                "Cache-Control:" => request_header.cache_control = String::from_str(key_value[1]),
                "Sec-WebSocket-Key:" => request_header.sec_websocket_key = String::from_str(key_value[1]),
                "Sec-WebSocket-Version:" => request_header.sec_websocket_version = String::from_str(key_value[1]),
                "Sec-WebSocket-Extensions:" => request_header.sec_websocket_extensions = String::from_str(key_value[1]),
                "User-Agent:" => request_header.user_agent = String::from_str(key_value[1]),
                _ => { /* Do nothing */ }
            }
        }

        return request_header;
    }

    // TODO - Create real logic that will actually verifiy this
    pub fn is_valid(&self) -> bool {
        if self.sec_websocket_key.as_slice() != "" {
            return true;
        }
        return false;
    }
}

impl ReturnHeader {

    /*
     * Returns the WebSocket Protocol Accept Header
     *
     * Accept Header:
     * HTTP/.1 101 Switching Protocols
     * Upgrade: websocket
     * Connection: Upgrade
     * Sec-WebSocket-Accept: COMPUTED_VALUE
     *
     * Steps to create the Sec-WebSocket-Accept Key:
     * 1.) Append "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" to passed key
     * 2.) SHA-1 Hash that value
     * 3.) Base64 encode the hashed bytes, not string
     * 4.) Return Base64 encoded bytes, not string
     */
    pub fn new(key: &str) -> ReturnHeader {
        // Combine key and WebSocket Key API thing
        let mut pre_hash = String::from_str(key);
        pre_hash.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");

        // Get the SHA-1 Hash as bytes
        let mut out = [0u8; 20];
        ReturnHeader::sha1_hash(pre_hash.as_slice(), &mut out);

        // Base64 encode the buffer
        let config = STANDARD;
        let encoded = out.to_base64(config);

        ReturnHeader {
            heading: String::from_str("HTTP/1.1 101 Switching Protocols\r\n"),
            upgrade: String::from_str("Upgrade: websocket\r\n"),
            connection: String::from_str("Connection: Upgrade\r\n"),
            sec_websocket_accept: encoded
        }
    }

    // Returns a string version of the header
    pub fn to_string(&self) -> String {
        let mut stringified = String::new();
        stringified.push_str(self.heading.as_slice());
        stringified.push_str(self.upgrade.as_slice());
        stringified.push_str(self.connection.as_slice());
        stringified.push_str("Sec-Websocket-Accept: ");
        stringified.push_str(self.sec_websocket_accept.as_slice());
        stringified.push_str("\r\n\r\n");
        return stringified;
    }

    /*
     * SHA-1 Hash performed on passed value
     * Bytes placed in passed out buffer
     */
    fn sha1_hash(value: &str, out: &mut [u8]) {
        let mut sha = box Sha1::new();
        (*sha).input_str(value);
        sha.result(out);
    }
}
