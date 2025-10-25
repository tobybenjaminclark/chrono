use dotenvy::dotenv;
use serde_json::json;
use crate::generators::gen_names::gen_characters;
use crate::generators::gen_places::fetch_map;
use crate::io::io::{read_map_from_file, write_map_to_file};
use crate::types::{ownership_to_json_map, Event};
use crate::utils::cluster::cluster_locations;
use crate::visualisers::viz_places::viz_map;

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
    let ownership = cluster_locations(&map);

    viz_map(&map, &ownership).unwrap();

    let ownership_map = ownership_to_json_map(ownership);


    // Build JSON
    let json_output = json!({
        "INIT_MAP": {
            "map": map,
            "characters": characters,
            "ownership": ownership_map,
            "events": vec!["temp".to_string()]
        }
    });

    // Convert to string
    json_output.to_string()

}


pub fn generate_start_events() -> Vec<Event> {
    // read array from start_events.json
    // each array is a list of three Event objects
    // pick a random one and return it

    todo!()
}