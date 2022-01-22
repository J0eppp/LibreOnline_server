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
    pub killed: bool,
}

impl Client {
    pub fn send(&mut self, msg: &[u8]) -> usize{
        match self.stream.write(msg) {
            Ok(size) => {
                // Message sent, flush
                match self.flush() {
                    Ok(_) => (),
                    Err(error) => {
                        println!("Error: {error}");
                    }
                }
                println!("Wrote {} bytes to client {}", size, self.id);
                size
            }
            Err(error) => {
                println!("Error: {error}");
                let err_code = error.raw_os_error().unwrap();
                if err_code == 32 {
                    // Broken pipe, aka client disconnected
                    self.killed = true;
                }
                0
            }
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        self.stream.flush()
    }

    pub fn handle_client(&mut self) -> Result<()> {
        let id = self.id;
        let _ = self.send(format!("Hi client {id}\n").as_bytes());

        let mut data = [0_u8; MESSAGE_BUFFER_SIZE];
        'thread_loop: while match self.stream.read(&mut data) {
            Ok(size) => {
                let _ = self.send(b"Received\n");

                let received_data = match from_utf8(&data) {
                    Ok(res) => res,
                    Err(err) => {
                        println!("Error: {err}");
                        &""
                    }
                };

                let received_data = strip_trailing_newline(received_data);
                match self.handle_message(received_data, size) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Error: {err}");
                    }
                }
                if self.killed {
                    break 'thread_loop;
                }
                true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    self.stream.peer_addr().unwrap()
                );
                // self.stream.shutdown(Shutdown::Both).unwrap();
                self.shutdown();
                false
            }
        } {}
        Ok(())
    }

    pub fn handle_message(&mut self, msg: &str, len: usize) -> Result<()> {
        print!("Received {len} bytes: {msg}");
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
        self.killed = true;
    }

}
