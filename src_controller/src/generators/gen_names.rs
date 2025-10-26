use std::fs;
use rand::seq::IndexedRandom;
use crate::types::{Character};

pub fn gen_characters() -> Vec<Character> {
    // Load the JSON file
    let data = fs::read_to_string("names.json")
        .expect("Failed to read names.json file");

    // Deserialize into a Vec<Character>
    let all_characters: Vec<Character> = serde_json::from_str(&data)
        .expect("Failed to parse JSON");

    // Filter by faction
    let gnomes: Vec<Character> = all_characters
        .iter()
        .filter(|c| c.faction == "g")
        .cloned()
        .collect();

    let trolls: Vec<Character> = all_characters
        .iter()
        .filter(|c| c.faction == "t")
        .cloned()
        .collect();

    let centaurs: Vec<Character> = all_characters
        .iter()
        .filter(|c| c.faction == "c")
        .cloned()
        .collect();


    let mut rng = rand::thread_rng();

    // Randomly select 4 of each faction
    let mut selected = Vec::new();
    selected.extend(gnomes.choose_multiple(&mut rng, 4).cloned());
    selected.extend(trolls.choose_multiple(&mut rng, 4).cloned());
    selected.extend(centaurs.choose_multiple(&mut rng, 4).cloned());

    selected
}
