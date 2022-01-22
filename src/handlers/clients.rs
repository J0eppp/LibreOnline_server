use std::net::{TcpStream, Shutdown};
use std::io::{self, Read, Write, Result};
use std::str::from_utf8;

use crate::handlers::messages::handle_message;


fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}


pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let hi_data = b"Hi\n";
    let _size = stream.write(hi_data).expect("Failed to say Hi to cliemt");

    if _size < hi_data.len() {
        // Failed to say hi
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            format!("Sent {}/{} bytes", _size, hi_data.len()),
        ));
    }

    stream.flush()?;

    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // stream.write(&data[0..size]).unwrap();
            stream.write(b"Received\n").unwrap();
            let received_data = from_utf8(&data).unwrap();
            let received_data = strip_trailing_newline(received_data);
            match handle_message(received_data, size) {
                Ok(_) => {
                    ()
                }
                Err(err) => {
                    println!("Error: {err}")
                }
            }
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
    Ok(())
}