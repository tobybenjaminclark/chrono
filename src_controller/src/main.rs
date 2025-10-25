mod generators;
mod io;
mod utils;
mod visualisers;
mod types;

use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::{env, thread};
use dotenvy::dotenv;
use serde_json::json;
use crate::generators::gen_names::gen_characters;
use crate::generators::gen_places::fetch_map;
use crate::io::client::handle_client;
use crate::io::io::{read_map_from_file, write_map_to_file};
use crate::types::ownership_to_json_map;
use crate::utils::cluster::cluster_locations;
use crate::visualisers::viz_places::viz_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_connection().await;
    Ok(())
}

pub async fn init_map(name: String, live: bool) -> String {

    let map = {
        if live {
            dotenv().ok();
            println!("Fetching up to {} attractions in {}...", 10, name);
            // Destructure the returned tuple
            let map = fetch_map(&*name, 10, 200.0).await.unwrap();
            println!("{}", map);

            let _ = write_map_to_file(&map, "map.json");

            map
        }
        else {
            read_map_from_file("map.json").unwrap()
        }
    };
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
