use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio_native_tls::TlsConnector;
use native_tls::TlsConnector as NativeTlsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = NativeTlsConnector::new()?;
    let connector = TlsConnector::from(connector);

    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:12345").await?;
    let mut tls_stream = connector.connect("localhost", stream).await?;

    let mut buf = vec![0; 1024];
    let n = tls_stream.read(&mut buf).await?;
    println!("Received: {}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
