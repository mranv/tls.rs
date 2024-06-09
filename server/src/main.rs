use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls::ServerConfig};
use tokio_rustls::rustls::{NoClientAuth, ServerConfig, PrivateKey, Certificate};
use tokio_stream::StreamExt;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufReader};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the keys and certificates
    let certs = load_certs("server.crt")?;
    let key = load_private_key("server.key")?;

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let tls_stream = acceptor.accept(stream).await.unwrap();
            let (mut reader, mut writer) = tokio::io::split(tls_stream);

            // Send data to the client in real-time
            tokio::spawn(async move {
                loop {
                    let data = "Hello from server!\n";
                    writer.write_all(data.as_bytes()).await.unwrap();
                    writer.flush().await.unwrap();
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            });

            let mut buffer = [0; 1024];
            while let Ok(n) = reader.read(&mut buffer).await {
                if n == 0 { break; }
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
            }
        });
    }
}

fn load_certs(path: &str) -> Result<Vec<Certificate>, Box<dyn Error>> {
    let certfile = File::open(path)?;
    let mut reader = BufReader::new(certfile);
    let certs = rustls_pemfile::certs(&mut reader)?
        .into_iter()
        .map(Certificate)
        .collect();
    Ok(certs)
}

fn load_private_key(path: &str) -> Result<PrivateKey, Box<dyn Error>> {
    let keyfile = File::open(path)?;
    let mut reader = BufReader::new(keyfile);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;

    if keys.len() != 1 {
        return Err("Expected a single private key".into());
    }

    Ok(PrivateKey(keys[0].clone()))
}
