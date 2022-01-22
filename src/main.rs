use std::net::TcpListener;
use std::thread;

mod handlers;

use crate::handlers::clients::handle_client;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
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
