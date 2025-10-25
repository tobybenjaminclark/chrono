mod generators;
mod io;
mod utils;
mod visualisers;
mod types;

use std::error::Error;
use std::net::TcpListener;
use std::thread;
use serde_json::json;
use crate::generators::gen_names::gen_characters;
use crate::io::client::handle_client;
use crate::io::io::read_map_from_file;
use crate::types::ownership_to_json_map;
use crate::utils::cluster::cluster_locations;
use crate::visualisers::viz_places::viz_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_connection();
    Ok(())
}

pub fn init_map(name: String) -> String {
    /*dotenv().ok();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <PLACE_NAME> <NUMBER_OF_RESULTS>");
        std::process::exit(1);
    }
    let place = &args[1];
    let n: usize = args[2].parse()?;
    println!("Fetching up to {} attractions in {}...", n, place);
    // Destructure the returned tuple
    let map = fetch_map(place, n, 200.0).await?;
    println!("{}", map);

    let _ = write_map_to_file(&map, "map.json");*/

    let map = read_map_from_file("map.json").unwrap();
    let characters = gen_characters();
    for character in &characters {
        println!("name: {}. faction: {:?}", character.name, character.faction);
    }
    let event = cluster_locations(&map);

    viz_map(&map, &event).unwrap();

    let ownership_map = ownership_to_json_map(&event);


    // Build JSON
    let json_output = json!({
        "INIT_MAP": {
            "map": map,
            "characters": characters,
            "ownership": ownership_map,
        }
    });

    // Convert to string
    json_output.to_string()

}

pub fn init_connection() {
    let port = "9999"; // You can change this
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .expect("Failed to bind to localhost");

    println!("Server listening on localhost:{}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                // Handle each client in a separate thread
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
