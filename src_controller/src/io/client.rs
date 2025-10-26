use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use serde_json::{json, Value};
use crate::endpoints::init_map;
use crate::generators::gen_events::gen_event;
use crate::types::{Character, Event};

pub async fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    let mut accumulated = String::new(); // persistent string buffer

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buffer[..n]);
                accumulated.push_str(&chunk);

                // Try to extract complete JSON messages
                loop {
                    // Find first '{' and the matching '}'
                    if let (Some(start), Some(end)) = (accumulated.find('{'), accumulated.rfind('}')) {
                        if start < end {
                            let candidate = &accumulated[start..=end];

                            // Try to parse candidate JSON
                            match serde_json::from_str::<Value>(candidate) {
                                Ok(parsed_json) => {
                                    println!("Parsed JSON: {}", parsed_json);

                                    // remove parsed message from buffer
                                    accumulated.replace_range(..=end, "");

                                    // Handle message
                                    if let Some(init_map_obj) = parsed_json.get("INIT_MAP") {
                                        let name = init_map_obj
                                            .get("loc_str")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("default");

                                        let response_json = init_map(name.to_string(), false).await;

                                        if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                                            eprintln!("Failed to send INIT_MAP response: {}", e);
                                            break;
                                        }

                                        continue;
                                    }
                                    else if let Some(gen_events_obj) = parsed_json.get("GEN_EVENTS") {

                                        println!("Events: {:?}", gen_events_obj
                                            .get("events"));

                                        let events: Vec<Event> = gen_events_obj
                                            .get("events")
                                            .and_then(|v| serde_json::from_value(v.clone()).ok())
                                            .unwrap_or_else(|| vec![]);


                                        let characters: Vec<Character> = gen_events_obj
                                            .get("characters")
                                            .and_then(|v| serde_json::from_value(v.clone()).ok())
                                            .unwrap_or_else(|| vec![]);

                                        println!("GEN_EVENTS requested for: {} events", n);

                                            let (sat, new_events) = gen_event(events.clone(), characters.clone()).await;
                                            println!("Generated: {:?}", new_events);

                                        let response = json!({
                                            "GEN_EVENTS": { "sat": sat, "_events": new_events }
                                        }).to_string();

                                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                                            eprintln!("Failed to send GEN_EVENTS response: {}", e);
                                            break;
                                        }

                                        continue;
                                    }
                                }
                                Err(e) => {
                                    // Incomplete JSON, wait for more data
                                    if e.is_eof() {
                                        break; // need more bytes, exit loop and read again
                                    } else {
                                        eprintln!("Failed to parse JSON: {}", e);
                                        // Skip malformed data
                                        accumulated.replace_range(..=end, "");
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
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
