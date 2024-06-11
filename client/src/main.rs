use std::net::TcpStream;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
