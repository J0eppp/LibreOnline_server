use std::io::Result;

pub fn handle_message(msg: &str, len: usize) -> Result<()> {
    print!("Received {len} bytes: {msg}");
    Ok(())
}