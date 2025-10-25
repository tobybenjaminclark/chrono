mod generators;
mod io;
mod utils;
mod visualisers;
mod types;
mod endpoints;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::io::client::handle_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_connection().await;
    Ok(())
}


pub async fn init_connection() {
    let port = "9999";
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .expect("Failed to bind to port");

    println!("Server listening on localhost:{}", port);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection from {}", addr);
                tokio::spawn(handle_client(stream));
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}
