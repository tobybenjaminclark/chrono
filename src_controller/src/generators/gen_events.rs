use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::prelude::{IndexedRandom, SliceRandom};

use crate::interval::plot::add_constraint_and_get_interval;
use crate::solver::solve::isPossible;
use crate::types::{Character, Effect, Event};
use crate::utils::prompt::get_name_and_description;

/// Always succeeds by inserting NEW_EVENT before the earliest reachable node in the DAG
fn safe_prepend(events: &mut Vec<Event>) -> (Vec<String>, (f32, f32), i32) {
    // Find the event with no incoming edges (earliest in topological order)
    let mut with_incoming: std::collections::HashSet<String> = std::collections::HashSet::new();
    for e in events.iter() {
        for b in &e.before {
            with_incoming.insert(b.clone());
        }
    }

    let candidate = events
        .iter()
        .find(|e| !with_incoming.contains(&e.name))
        .cloned();

    if let Some(target) = candidate {
        let end_time = if target.start > 0.05 { target.start } else { 0.05 };
        let before_list = vec![target.name.clone()];
        ((before_list), (0.0, end_time as f32), 0)
    } else {
        // In degenerate case (all events interdependent), create isolated event
        (vec![], (0.0, 0.1), 0)
    }
}

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
            events[a_idx].before.retain(|b| b != target_b);
            events[a_idx].before.push("NEW_EVENT".to_string());
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

pub async fn gen_event(
    existing_events: Vec<Event>,
    existing_characters: Vec<Character>,
) -> (bool, Vec<Event>) {
    let mut rng = StdRng::from_rng(&mut rand::thread_rng());
    let types = vec!["auxiliary", "catastrophe", "ceremony", "catastrophe", "miracle", "catastrophe", "catastrophe", "catastrophe", "catastrophe", "catastrophe", "catastrophe", "catastrophe"];
    let event_type = types.choose(&mut rng).unwrap().to_string();

    // Possible effects
    let event_effects: Vec<Effect> = match event_type.as_str() {
        "catastrophe" => {
            if !existing_characters.is_empty() && rng.random_bool(0.8) {
                let victim = existing_characters.choose(&mut rng).unwrap().clone();
                vec![Effect::Death(victim.name)]
            } else {
                vec![]
            }
        }
        _ => vec![],
    };

    if existing_events.is_empty() {
        panic!("No existing events to place this event before!");
    }

    // --- Retry up to N times to find a valid insertion ---
    let mut attempt = 0;
    let max_attempts = 5;
    let mut interval = (-1.0, -1.0);
    let mut track = -1;
    let mut updated_events = existing_events.clone();
    let mut before_list = vec!["".to_string()];
    let mut before_event_name = "".to_string();

    while attempt < max_attempts {
        attempt += 1;
        before_event_name = existing_events.choose(&mut rng).unwrap().name.clone();

        let (candidate_events, candidate_before) =
            maybe_transitive_insert(existing_events.clone(), &mut rng, &before_event_name);

        match add_constraint_and_get_interval(
            candidate_events.clone(),
            ("NEW_EVENT", &candidate_before[0]),
            "intervals.png",
        ) {
            Ok((iv, tr, up)) => {
                interval = iv;
                track = tr;
                updated_events = up;
                before_list = candidate_before;
                println!("✅ Inserted after {} on attempt {}", before_event_name, attempt);
                break;
            }
            Err(e) => {
                eprintln!("⚠️ Attempt {} failed: {}", attempt, e);
                // reset before trying again
                interval = (-1.0, -1.0);
                track = -1;
            }
        }
    }


    // --- Fallback: prepend before earliest event ---
    if interval.0 < 0.0 || track < 0 {
        eprintln!("⚠️ All attempts failed; prepending before earliest event.");
        let (before_list_, interval_, track_) = safe_prepend(&mut existing_events.clone());
        before_list = before_list_;
        interval = (interval_.0 as f64, interval_.1 as f64);
        track = track_;
        updated_events = existing_events.clone();
    }



    // --- Character involvement ---
    let characters: Vec<Character> = if let Some(Effect::Death(name)) =
        event_effects.iter().find(|e| matches!(e, Effect::Death(_)))
    {
        // Death event → include the victim
        existing_characters
            .iter()
            .find(|ch| &ch.name == name)
            .cloned()
            .map(|ch| vec![ch])
            .unwrap_or_else(Vec::new)
    } else if !existing_characters.is_empty() {
        let roll: f32 = rng.r#gen(); // 0.0 → 1.0
        let num_chars = if roll < 0.8 {
            3 // 40%
        } else if roll < 0.9{
            1 // 30%
        } else {
            0
        };

        if num_chars > 0 {
            // --- Pick a random faction among existing characters ---
            let faction = existing_characters
                .choose(&mut rng)
                .map(|ch| ch.faction.clone())
                .unwrap_or_default();

            // --- Filter characters by that faction ---
            let mut same_faction_chars: Vec<Character> = existing_characters
                .iter()
                .filter(|ch| ch.faction == faction)
                .cloned()
                .collect();

            // --- Shuffle and truncate to desired size ---
            same_faction_chars.shuffle(&mut rng);
            same_faction_chars.truncate(num_chars.min(same_faction_chars.len()));
            same_faction_chars
        } else {
            vec![]
        }
    } else {
        vec![]
    };


    // --- Construct event ---
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

    let event = get_name_and_description(event).await.unwrap();

    let combined: Vec<Event> = updated_events.into_iter().chain(std::iter::once(event)).collect();
    let sat = isPossible(combined.clone(), existing_characters);

    (sat, combined)
}

/// Helper: estimate earliest event's end time
fn first_event_opt_end(events: &[Event]) -> f32 {
    events
        .iter()
        .map(|e| e.end)
        .fold(f32::INFINITY, |a, b| a.min(b as f32))
        .max(0.05) // ensure >0
}