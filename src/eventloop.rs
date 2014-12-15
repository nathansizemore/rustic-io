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


//use std::str;
use std::rand;
use std::rand::Rng;
use std::io::TcpStream;
use std::comm::TryRecvError;

use super::action::Action;
use super::message::Message;
use super::event::Event;
use super::socket::Socket;


/*
 * Represents a socket and the message channel to that socket
 */
pub struct SocketMessenger {
    pub id: String,
    pub to_socket: Sender<Message>
}


/*
 *
 */
pub fn start(action_sender: Sender<Action>, action_receiver: Receiver<Action>,
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
                let mut socket = Socket {
                    id: id.clone(),
                    stream: stream.clone(),
                    to_event_loop: action_sender.clone(),
                    to_write_task: socket_sender.clone(), // This just here because we need some value
                    events: events.clone()
                };

                // Add socket and channel to messenger vector
                socket_msngers.push(SocketMessenger {
                    id: socket.id.clone(),
                    to_socket: socket_sender.clone()
                });

                // Start a new socket
                spawn(move || {
                    socket.start(from_event_loop_recvr);
                });
            }
            Err(e) => { /* Dont care */ }
        }

        // Check for incoming actions to take
        match action_receiver.try_recv() {
            Ok(action) => {
                match action.event.as_slice() {
                    "broadcast" => {
                        for msnger in socket_msngers.iter() {
                            msnger.to_socket.send(action.message.clone());
                        }
                    }
                    "drop_socket" => {
                        let mut counter = 0;
                        let mut index: i32 = -1;
                        for msnger in socket_msngers.iter() {
                            if msnger.id.as_slice() == action.socket_id.as_slice() {
                                index = counter;
                                break;
                            }
                            counter += 1;
                        }

                        if index >= 0 {
                            socket_msngers.remove(index as uint);
                        }
                    }
                    _ => { /* Do nothing */ }
                }
            }
            Err(e) => {
                match e {
                    TryRecvError::Disconnected => {
                        println!("action_receiver disconnected...");
                        // Channel is disconnected, kill stuff
                        
                        // TODO - panic and start killing stuff
                    }
                    TryRecvError::Empty => { /* Dont care */ }
                }
            }
        }
    }
}

/*
 * 
 */
fn generate_socket_id() -> String {
    let mut rng = rand::task_rng();
    let mut string = String::new();

    for x in range(0i, 15i) {
        string.push(rng.gen::<char>());
    }
    string
}










