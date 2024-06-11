use std::net::{TcpListener, TcpStream};
use std::io::{Write, Result};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let response = b"Hello from server!";
    stream.write_all(response)?;
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream)?;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}
