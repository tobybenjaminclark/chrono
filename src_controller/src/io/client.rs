use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use serde_json::Value;
use crate::init_map;

pub async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                println!("New message");
                let raw_bytes = &buffer[..n];
                let raw_str = String::from_utf8_lossy(raw_bytes);

                if let (Some(start), Some(end)) = (raw_str.find('{'), raw_str.rfind('}')) {
                    if start < end {
                        let json_str = raw_str[start..=end].trim_end_matches(|c: char| !c.is_ascii_graphic());
                        println!("Extracted JSON: {}", json_str);

                        match serde_json::from_str::<Value>(json_str) {
                            Ok(parsed_json) => {
                                println!("Parsed JSON: {}", parsed_json);

                                if let Some(init_map_obj) = parsed_json.get("INIT_MAP") {
                                    let name = init_map_obj.get("loc_str")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("default");

                                    println!("INIT_MAP requested for name: {}", name);

                                    let response_json = init_map(name.to_string(), true).await;

                                    if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                                        eprintln!("Failed to send INIT_MAP response: {}", e);
                                        break;
                                    }

                                    continue;
                                }
                            }
                            Err(e) => println!("Failed to parse JSON: {}, error: {}", json_str, e),
                        }

                        if let Err(e) = stream.write_all(json_str.as_bytes()).await {
                            eprintln!("Failed to send response: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client {}: {}", stream.peer_addr().unwrap(), e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = "9999";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    println!("Server listening on localhost:{}", port);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);
        tokio::spawn(handle_client(stream));
    }
}
