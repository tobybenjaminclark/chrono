use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Received from {}: {}", stream.peer_addr().unwrap(), received);

                // Echo the message back to the client
                if let Err(e) = stream.write_all(received.as_bytes()) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from client {}: {}", stream.peer_addr().unwrap(), e);
                break;
            }
        }
    }
}