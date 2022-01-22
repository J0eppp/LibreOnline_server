use std::io::{Result, Write, Read};
use std::net::TcpStream;
use std::net::Shutdown;
use std::str::from_utf8;

use crate::constraints::MESSAGE_BUFFER_SIZE;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(input)
}

pub struct Client {
    pub id: u8,
    pub stream: TcpStream,
}

impl Client {
    pub fn send(&mut self, msg: &[u8]) -> Result<usize> {
        self.stream.write(msg)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }

    pub fn handle_client(&mut self) -> Result<()> {
        let hi_data = b"Hi client x\n";
        match self.send(hi_data) {
            Ok(size) => {
                // Message sent, flush
                match self.flush() {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Error: {error}");
                    }
                }
                println!("Wrote {} bytes to client {}", size, self.id);
            }
            Err(error) => {
                println!("Error: {error}");
            }
        }

        let mut data = [0_u8; MESSAGE_BUFFER_SIZE];
        while match self.stream.read(&mut data) {
            Ok(size) => {
                // Send received, unnecessary but here for testing
                match self.send(b"Received\n") {
                    Ok(size) => {
                        println!("Wrote {} bytes to client {}", size, self.id);
                    }
                    Err(error) => {
                        println!("Error: {error}");
                    }
                }

                let received_data = match from_utf8(&data) {
                    Ok(res) => res,
                    Err(error) => {
                        println!("Error: {error}");
                        &""
                    }
                };

                let received_data = strip_trailing_newline(strip_trailing_newline(received_data));
                match self.handle_message(received_data, size) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Error: {err}")
                    }
                }
                true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    self.stream.peer_addr().unwrap()
                );
                self.stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
        Ok(())
    }

    pub fn handle_message(&mut self, msg: &str, len: usize) -> Result<()> {
        print!("Received {len} bytes: {msg}");
        Ok(())
    }

}
