use rand::prelude::IndexedRandom;
use rand::Rng;
use crate::interval::plot::add_constraint_and_get_interval;
use crate::types::{Character, Event};
pub fn gen_event(
    existing_events: Vec<Event>,
    existing_characters: Vec<Character>,
) -> Event {
    let mut rng = rand::thread_rng();

    // Possible event types and effects
    let types = vec!["auxiliary", "catastrophe"];
    let effects = vec!["death"];

    // Randomly pick a type
    let event_type = types.choose(&mut rng).unwrap().to_string();

    // Determine possible effects
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
    let mut before_list = vec![before_event.name.clone()];

    let full_events: Vec<Event> = existing_events.clone(); // all previously added events

    // Add constraint and get interval
    let constraints: Vec<(&str, &str)> = full_events
        .iter()
        .flat_map(|e| {
            e.before
                .iter()
                .map(|b| (e.name.as_str(), b.as_str()))
                .collect::<Vec<_>>()
        })
        .collect();




    println!("Existing constraints: {:?}", constraints);
    println!("Trying to add event, before {:?}", before_event.name);

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



    // Determine characters affected
    let characters = if event_effects.contains(&"death".to_string()) && !existing_characters.is_empty() {
        vec![existing_characters.choose(&mut rng).unwrap().clone()]
    } else {
        vec![]
    };

    Event {
        name: format!("Event_{}", rng.r#gen::<u32>()),
        description: format!("A {} event.", event_type),
        before: before_list,
        start: interval.0,
        end: interval.1,
        _type: event_type,
        characters,
        effects: event_effects,
    }
}