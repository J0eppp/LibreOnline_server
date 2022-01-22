use std::net::TcpListener;
use std::thread;

mod types;
mod constraints;

use crate::types::client::Client;
use crate::constraints::MAX_CLIENTS;

fn main() {
    let mut amount_of_clients: u8 = 0;
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if amount_of_clients >= MAX_CLIENTS {
                    // Nope
                    drop(stream);
                    return;
                }
                let mut client = Client {
                    id: amount_of_clients,
                    stream,
                };

                println!("New connection: {}", client.stream.peer_addr().unwrap());
                amount_of_clients += 1;
                thread::spawn(move || {
                    // connection succeeded
                    client.handle_client()
                });
            }
            Err(e) => {
                // Connection failed
                println!("Error: {}", e);
            }
        }
    }
    // Close the socket server
    drop(listener);
}
