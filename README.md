# tls.rs : Secure TCP Client-Server in Rust

This repository contains a simple implementation of a secure TCP client-server application in Rust. The communication between the client and the server is secured using SSL/TLS with public/private key configuration. The server sends data to the client in real-time, and the client prints this data as it is received.

## Features

- Asynchronous TCP communication using `tokio`.
- SSL/TLS encryption using `native-tls` and `tokio-native-tls`.
- Public/private key authentication.
- Real-time data transmission from server to client.

## Requirements

- Rust (latest stable version)
- OpenSSL (for generating certificates)

## Setup

### Generate Certificates

To run the server and client, you need to generate the necessary certificates and keys. You can use OpenSSL to generate these files.

1. Generate a private key:

   ```sh
   openssl genrsa -out key.pem 2048
   ```

2. Generate a certificate signing request (CSR):

   ```sh
   openssl req -new -key key.pem -out cert.csr
   ```

3. Generate a self-signed certificate:

   ```sh
   openssl x509 -req -days 365 -in cert.csr -signkey key.pem -out cert.pem
   ```

4. Convert the key and certificate to PKCS#12 format:

   ```sh
   openssl pkcs12 -export -out identity.pfx -inkey key.pem -in cert.pem
   ```

   Use `password` when prompted for an export password.

### Running the Server

1. Navigate to the `server` directory:

   ```sh
   cd server
   ```

2. Ensure you have the required dependencies:

   ```sh
   cargo build --release
   ```

3. Run the server:

   ```sh
   cargo run --release
   ```

### Running the Client

1. Navigate to the `client` directory:

   ```sh
   cd client
   ```

2. Ensure you have the required dependencies:

   ```sh
   cargo build --release
   ```

3. Run the client:

   ```sh
   cargo run --release
   ```

## Project Structure

```
tls.rs
├── certs
│   ├── cert.csr
│   ├── cert.pem
│   ├── identity.pfx
│   └── key.pem
├── client
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── README.md
└── server
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── main.rs
```

- `certs/`: Directory containing the certificate and key files.
- `client/`: Directory containing the client implementation.
  - `Cargo.toml`: Client dependencies.
  - `src/main.rs`: Main client implementation.
- `server/`: Directory containing the server implementation.
  - `Cargo.toml`: Server dependencies.
  - `src/main.rs`: Main server implementation.
- `README.md`: This readme file.
