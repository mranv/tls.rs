use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio_native_tls::TlsAcceptor;
use native_tls::{Identity, TlsAcceptor as NativeTlsAcceptor};
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load server's identity
    let mut file = File::open("/home/mranv/Desktop/d3v/tls.rs/certs/identity.pfx")?;
    let mut identity = vec![];
    file.read_to_end(&mut identity)?;
    let identity = Identity::from_pkcs12(&identity, "password")?;
    
    // Create a TLS acceptor
    let native_acceptor = NativeTlsAcceptor::builder(identity).build()?;
    let acceptor = TlsAcceptor::from(native_acceptor);

    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:12345").await?;
    println!("Server listening on 127.0.0.1:12345");

    loop {
        let (socket, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        tokio::spawn(async move {
            let mut tls_stream = acceptor.accept(socket).await.unwrap();
            tls_stream.write_all(b"Hello from server!").await.unwrap();
            tls_stream.shutdown().await.unwrap();
        });
    }
}
