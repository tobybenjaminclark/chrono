mod gen_places;
mod viz_places;
mod types;
mod cluster;
mod io;
mod client;

use std::env;
use std::error::Error;
use dotenvy::dotenv;
use crate::cluster::cluster_locations;
use crate::gen_places::fetch_map;
use crate::io::{read_map_from_file, write_map_to_file};
use crate::viz_places::viz_map;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let event = cluster_locations(&map);

    viz_map(&map, &event).unwrap();

    Ok(())
}
