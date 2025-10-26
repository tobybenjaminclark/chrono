use rand::Rng;
use rand::rngs::{StdRng, OsRng};
use rand::SeedableRng;
use rand::prelude::IndexedRandom;

use crate::interval::plot::add_constraint_and_get_interval;
use crate::solver::solve::isPossible;
use crate::types::{Character, Effect, Event};
use crate::utils::prompt::get_name_and_description;

/// Performs a 75% chance of transitive insertion (A < NEW < B),
/// otherwise leaves a simple NEW < B relationship.
///
/// Returns the updated events and the "before" list for the new event.
fn maybe_transitive_insert(
    mut events: Vec<Event>,
    rng: &mut StdRng,
    before_event_name: &str,
) -> (Vec<Event>, Vec<String>) {
    let mut before_list = vec![before_event_name.to_string()];


    if rng.gen_bool(0.75) {
        let target_b = before_event_name;
        if let Some(a_idx) = events.iter().position(|e| e.before.contains(&target_b.to_string())) {
            let a_name = events[a_idx].name.clone();

            // rewire A.before: replace B with NEW_EVENT
            events[a_idx].before.retain(|b| b != target_b);
            events[a_idx].before.push("NEW_EVENT".to_string());

            // new event should go before B
            before_list = vec![target_b.to_string()];

            println!("🔁 Transitive insertion: {} < NEW_EVENT < {}", a_name, target_b);
        } else {
            println!("ℹ️ No suitable transitive pair found; flat insertion instead.");
        }
    } else {
        println!("ℹ️ Flat insertion (no transitive rewiring).");
    }

    (events, before_list)
}

/// Generates a new event and inserts it into the timeline.
pub async fn gen_event(
    existing_events: Vec<Event>,
    existing_characters: Vec<Character>,
) -> (bool, Vec<Event>) {
    // --- RNG setup ---
    let mut rng = StdRng::from_rng(&mut rand::thread_rng());

    // --- Event type and effects ---
    let types = vec!["auxiliary", "catastrophe"];
    let event_type = types.choose(&mut rng).unwrap().to_string();

    let event_effects: Vec<Effect> = match event_type.as_str() {
        "catastrophe" => {
            if !existing_characters.is_empty() && rng.gen_bool(0.5) {
                let victim = existing_characters.choose(&mut rng).unwrap().clone();
                vec![Effect::Death(victim.name)]
            } else {
                vec![]
            }
        }
        _ => vec![],
    };

    // --- Pick random event to place before ---
    if existing_events.is_empty() {
        panic!("No existing events to place this event before!");
    }

    // copy the name so we avoid borrow conflicts
    let before_event_name = existing_events.choose(&mut rng).unwrap().name.clone();

    // --- Possibly perform transitive insertion ---
    let (updated_events_with_links, before_list) =
        maybe_transitive_insert(existing_events.clone(), &mut rng, &before_event_name);


    // --- Compute interval after adding constraint ---
    let (interval, track, updated_events) = match add_constraint_and_get_interval(
        updated_events_with_links.clone(),
        ("NEW_EVENT", &before_list[0]),
        "intervals.png",
    ) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("⚠️ Failed to get interval for {}: {}", before_event_name, e);
            ((-1.0, -1.0), -1, vec![])
        }
    };

    // --- Determine affected characters (e.g., deaths) ---
    let characters: Vec<Character> = if let Some(Effect::Death(name)) =
        event_effects.iter().find(|e| matches!(e, Effect::Death(_)))
    {
        existing_characters
            .iter()
            .find(|ch| &ch.name == name)
            .cloned()
            .map(|ch| vec![ch])
            .unwrap_or_else(Vec::new)
    } else {
        vec![]
    };

    // --- Construct new event ---
    let event = Event {
        name: "Event".into(),
        description: format!("A {} event.", event_type),
        before: before_list,
        start: interval.0,
        end: interval.1,
        _type: event_type,
        characters,
        effects: event_effects,
        track: 0.0,
    };

    // --- Prompt for generated name/description ---
    let event = get_name_and_description(event).await.unwrap();

    // --- Merge into full timeline ---
    let combined: Vec<Event> = updated_events.into_iter().chain(std::iter::once(event)).collect();

    // --- Validate ---
    let sat = isPossible(combined.clone(), existing_characters);
    (sat, combined)
}
