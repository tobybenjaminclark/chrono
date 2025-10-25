mod generators;
mod io;
mod utils;
mod visualisers;
mod types;

use std::error::Error;
use crate::generators::gen_names::gen_characters;
use crate::io::io::read_map_from_file;
use crate::utils::cluster::cluster_locations;
use crate::visualisers::viz_places::viz_map;

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
    let characters = gen_characters();
    for character in characters {
        println!("name: {}. faction: {:?}", character.name, character.faction);
    }
    let event = cluster_locations(&map);

    viz_map(&map, &event).unwrap();

    Ok(())
}
