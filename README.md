

# tls.rs : Secure TCP Client-Server in Rust

This repository contains a simple implementation of a secure TCP client-server application in Rust. The communication between the client and the server is secured using SSL/TLS with public/private key configuration. The server sends data to the client in real-time, and the client prints this data as it is received.

## Features

- Asynchronous TCP communication using `tokio`.
- SSL/TLS encryption using `rustls` and `tokio-rustls`.
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
    openssl genpkey -algorithm RSA -out server.key
    ```

2. Generate a certificate signing request (CSR):

    ```sh
    openssl req -new -key server.key -out server.csr
    ```

3. Generate a self-signed certificate:

    ```sh
    openssl x509 -req -days 365 -in server.csr -signkey server.key -out server.crt
    ```

4. Generate a CA certificate for the client:

    ```sh
    openssl req -x509 -new -nodes -key server.key -sha256 -days 1024 -out ca.crt
    ```

### Running the Server

1. Ensure you have the required dependencies:

    ```sh
    cargo build
    ```

2. Run the server:

    ```sh
    cargo run --bin server
    ```

### Running the Client

1. Ensure you have the required dependencies:

    ```sh
    cargo build
    ```

2. Run the client:

    ```sh
    cargo run --bin client
    ```

## Project Structure

- `src/main.rs`: Main server implementation.
- `src/client.rs`: Main client implementation.
- `server.crt`: Server certificate.
- `server.key`: Server private key.
- `ca.crt`: CA certificate for the client.
