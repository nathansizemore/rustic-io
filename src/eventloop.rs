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


use std::str;
use action::Action;
use std::io::TcpStream;
use std::comm::TryRecvError;


pub fn start(action_sender: Sender<Action>, action_receiver: Receiver<Action>
    new_conn_receiver: Receiver<TcpStream>, events: Vec<Event>) {

    // Vector of socket ids and senders to each of their listening tasks
    let mut socket_msngers: Vec<SocketMessenger> = Vec::new();

    loop {
        // Check for new connections
        match new_conn_receiver.try_recv() {
            Ok(stream) => {
                // Generate id, create channel, and socket
                let id = generate_socket_id();
                let (socket_sender, from_event_loop_recvr): (Sender<Message>, Receiver<Message>) = channel();
                let socket = Socket {
                    id: id.clone(),
                    stream: stream.clone(),
                    to_event_loop: action_sender.clone(),
                    events: events.clone()
                }

                // Add socket and channel to messenger vector
                socket_msngers.push(SocketMessenger {
                    id: socket.id.clone(),
                    to_socket: socket_sender.clone()
                });

                // Start a new socket
                socket.start(from_event_loop_recvr);
            }
            Err(e) => { /* Dont care */ }
        }

        // Check for incoming actions to take
        match action_receiver.try_recv() {
            Ok(action) => {
                match action.event.as_slice() {
                    "broadcast" => {
                        for msnger in socket_msngers.iter() {
                            msnger.to_socket.send(action.message.clone())
                        }
                    }
                    _ => {
                        println!("Action: {} is currently not implemented");
                    }
                }
            }
            Err(e) => {
                match e {
                    TryRecvError::Disconnected => {
                        // Channel is disconnected, kill stuff
                        
                        // TODO - panic and start killing stuff
                    }
                    TryRecvError::Empty => { /* Dont care */ }
                }
            }
        }
    }
}

fn generate_socket_id() -> String {
    let mut string = String::new();
    for char in rand::task_rng().gen_ascii_chars().take(15) {
        string.push_char(n);
    }
    string
}










