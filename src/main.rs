use std::net::{TcpListener};
use std::thread;

mod constants;
mod types;
mod db;

use crate::constants::MAX_CLIENTS_U8;
use crate::types::client::new_client;

fn main() {
    let mut amount_of_clients: u8 = 0;

    // Open a database
    let pool = match db::open() {
        Ok(conn) => conn,
        Err(err) => {
            println!("Error: {}", err);
            return
        }
    };

    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    db::setup(conn);

    

    

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if amount_of_clients >= MAX_CLIENTS_U8 {
                    // Nope
                    drop(stream);
                    return;
                }
                let mut client = new_client(amount_of_clients, stream);
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
