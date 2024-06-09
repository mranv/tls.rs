use tokio::net::TcpStream;
use tokio_rustls::{TlsConnector, rustls::ClientConfig};
use tokio_rustls::rustls::{Certificate, PrivateKey};
use tokio_stream::StreamExt;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufReader};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the root certificates
    let root_certs = load_certs("ca.crt")?;

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_certs)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let domain = rustls::ServerName::try_from("localhost")?;
    let tls_stream = connector.connect(domain, stream).await?;

    let (mut reader, mut writer) = tokio::io::split(tls_stream);

    // Reading data in real-time
    tokio::spawn(async move {
        let mut buffer = [0; 1024];
        while let Ok(n) = reader.read(&mut buffer).await {
            if n == 0 { break; }
            println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
        }
    });

    let message = "Hello from client!\n";
    writer.write_all(message.as_bytes()).await?;
    writer.flush().await?;
    
    Ok(())
}

fn load_certs(path: &str) -> Result<rustls::RootCertStore, Box<dyn Error>> {
    let certfile = File::open(path)?;
    let mut reader = BufReader::new(certfile);
    let certs = rustls_pemfile::certs(&mut reader)?
        .into_iter()
        .map(rustls::Certificate)
        .collect();

    let mut root_cert_store = rustls::RootCertStore::empty();
    root_cert_store.add_parsable_certificates(&certs);
    
    Ok(root_cert_store)
}
