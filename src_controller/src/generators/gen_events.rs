use rand::Rng;
use rand::rngs::{StdRng, OsRng};
use rand::SeedableRng;
use rand::prelude::IndexedRandom;

use crate::interval::plot::add_constraint_and_get_interval;
use crate::types::{Character, Event};
use crate::utils::prompt::get_name_and_description;

pub async fn gen_event(
    existing_events: Vec<Event>,
    existing_characters: Vec<Character>,
) -> Event {
    // Send-safe RNG
    let mut rng = StdRng::from_rng(&mut rand::thread_rng());

    // Possible event types and effects
    let types = vec!["auxiliary", "catastrophe"];
    let effects = vec!["death"];

    // Randomly pick a type
    let event_type = types.choose(&mut rng).unwrap().to_string();

    // Determine effects
    let event_effects = match event_type.as_str() {
        "catastrophe" => {
            if rng.gen_bool(0.5) {
                vec!["death".to_string()]
            } else {
                vec![]
            }
        }
        _ => vec![], // auxiliary events have no effects
    };

    // Pick a random existing event to be BEFORE
    if existing_events.is_empty() {
        panic!("No existing events to place this event before!");
    }
    let before_event = existing_events.choose(&mut rng).unwrap();
    let before_list = vec![before_event.name.clone()];

    // Build constraints for all existing events
    let constraints: Vec<(&str, &str)> = existing_events
        .iter()
        .flat_map(|e| {
            e.before
                .iter()
                .map(|b| (e.name.as_str(), b.as_str()))
                .collect::<Vec<_>>() // if before is empty, this will produce an empty Vec
        })
        .collect();

    println!("Existing constraints: {:?}", constraints);
    println!("Trying to add event before {:?}", before_event.name);

    // Add constraint and get interval
    let interval = match add_constraint_and_get_interval(
        constraints,
        ("NEW_EVENT", &before_event.name),
        "intervals.png",
    ) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("⚠️ Failed to get interval for {}: {}", before_event.name, e);
            (0.0, 1.0)
        }
    };

    // Determine affected characters
    let characters = if event_effects.contains(&"death".to_string()) && !existing_characters.is_empty() {
        vec![existing_characters.choose(&mut rng).unwrap().clone()]
    } else {
        vec![]
    };

    // Build the event
    let event = Event {
        name: format!("Event"),
        description: format!("A {} event.", event_type),
        before: before_list,
        start: interval.0,
        end: interval.1,
        _type: event_type,
        characters,
        effects: event_effects,
    };

    // Optional: prompt for name/description
    get_name_and_description(event).await.unwrap()
}
