use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use serde_json::{json, Value};
use crate::endpoints::init_map;
use crate::generators::gen_events::gen_event;
use crate::types::{Character, Event};

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

                                    let response_json = init_map(name.to_string(), false).await;

                                    if let Err(e) = stream.write_all(response_json.as_bytes()).await {
                                        eprintln!("Failed to send INIT_MAP response: {}", e);
                                        break;
                                    }

                                    continue;
                                }
                                else if let Some(gen_events_obj) = parsed_json.get("GEN_EVENTS") {
                                    let n = gen_events_obj.get("n")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(0);

                                    let events: Vec<Event> = gen_events_obj
                                        .get("events")
                                        .and_then(|v| serde_json::from_value(v.clone()).ok())
                                        .unwrap_or_else(|| vec![]);

                                    let characters: Vec<Character> = gen_events_obj
                                        .get("characters")
                                        .and_then(|v| serde_json::from_value(v.clone()).ok())
                                        .unwrap_or_else(|| vec![]);

                                    println!("GEN_EVENTS requested for: {} events", n);

                                    let mut new_events = Vec::new();

                                    for _ in 0..n {
                                        let e = gen_event(events.clone(), characters.clone());
                                        new_events.push(e);
                                    }

                                    let response = json!({
                                        "GEN_EVENTS": {
                                            "events": new_events,
                                        }
                                    }).to_string();


                                    if let Err(e) = stream.write_all(response.as_bytes()).await {
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
