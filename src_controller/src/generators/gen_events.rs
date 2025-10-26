use rand::Rng;
use rand::rngs::{StdRng, OsRng};
use rand::SeedableRng;
use rand::prelude::IndexedRandom;

use crate::interval::plot::add_constraint_and_get_interval;
use crate::solver::solve::isPossible;
use crate::types::{Character, Effect, Event};
use crate::utils::prompt::get_name_and_description;

pub async fn gen_event(
    existing_events: Vec<Event>,
    existing_characters: Vec<Character>,
) -> (bool, Vec<Event>) {
    // Send-safe RNG
    let mut rng = StdRng::from_rng(&mut rand::thread_rng());

    // Possible event types and effects
    let types = vec!["auxiliary", "catastrophe"];
    let effects = vec!["death"];

    // Randomly pick a type
    let event_type = types.choose(&mut rng).unwrap().to_string();

    // Determine effects
    let event_effects: Vec<Effect> = match event_type.as_str() {
        "catastrophe" => {
            if !existing_characters.is_empty() && rng.gen_bool(0.5) {
                let victim = existing_characters.choose(&mut rng).unwrap().clone();
                vec![Effect::Death(victim.name)]
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

    // Add constraint and get intervals
    let (interval, updated_events) = match add_constraint_and_get_interval(
        existing_events.clone(), // clone to transfer ownership
        ("NEW_EVENT", &before_event.name),
        "intervals.png",
    ) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("⚠️ Failed to get interval for {}: {}", before_event.name, e);
            ((-1.0, -1.0), vec![])
        }
    };


    // Determine affected characters
    let characters: Vec<Character> = if let Some(Effect::Death(name)) = event_effects.iter().find(|e| matches!(e, Effect::Death(_))) {
        existing_characters
            .iter()
            .find(|ch| &ch.name == name) // match by name
            .cloned()                    // clone the Character struct
            .map(|ch| vec![ch])          // wrap in a vec
            .unwrap_or_else(Vec::new)    // fallback to empty vec if not found
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

    // Prompt for name/description
    let event = get_name_and_description(event).await.unwrap();

    let combined: Vec<Event> = updated_events.into_iter().chain(std::iter::once(event)).collect();


    if isPossible(combined.clone()) {
        return (true, combined);
    } else {
        return (false, combined);
    }
}
