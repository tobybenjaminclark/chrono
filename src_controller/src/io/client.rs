use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::Value;
use crate::init_map; // make sure your init_map function takes a name

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let raw_bytes = &buffer[..n];
                let raw_str = String::from_utf8_lossy(raw_bytes);

                // Extract JSON from first '{' to last '}'
                if let (Some(start), Some(end)) = (raw_str.find('{'), raw_str.rfind('}')) {
                    if start < end {
                        let json_str = &raw_str[start..=end];
                        println!("Extracted JSON: {}", json_str);

                        match serde_json::from_str::<Value>(json_str) {
                            Ok(parsed_json) => {
                                println!("Parsed JSON: {}", parsed_json);

                                // Check if INIT_MAP exists
                                if let Some(init_map_obj) = parsed_json.get("INIT_MAP") {
                                    // Extract name (optional handling)
                                    let name = init_map_obj.get("name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("default");

                                    println!("INIT_MAP requested for name: {}", name);

                                    // Call init_map with the name
                                    let response_json = init_map(name.parse().unwrap());

                                    // Send the JSON back to the client
                                    if let Err(e) = stream.write_all(response_json.as_bytes()) {
                                        eprintln!("Failed to send INIT_MAP response: {}", e);
                                        break;
                                    }

                                    continue; // skip echoing original message
                                }
                            }
                            Err(e) => println!("Failed to parse JSON: {}, error: {}", json_str, e),
                        }

                        // Echo back the cleaned JSON if INIT_MAP wasn't present
                        if let Err(e) = stream.write_all(json_str.as_bytes()) {
                            eprintln!("Failed to send response: {}", e);
                            break;
                        }
                    } else {
                        println!("No valid JSON found in message");
                    }
                } else {
                    println!("No JSON braces found in message");
                }
            }
            Err(e) => {
                eprintln!("Error reading from client {}: {}", stream.peer_addr().unwrap(), e);
                break;
            }
        }
    }
}
