use     z3::{Config, Context, Solver, ast::{Int, Bool}, SatResult};
use crate::types::{Event, Character, Effect};
use std::collections::HashMap;

pub fn isPossible(events: Vec<Event>, chars: Vec<Character>) -> bool {
    let solver = Solver::new();

    // Map event names to Z3 integer variables
    let mut event_times: HashMap<String, Int> = HashMap::new();
    for e in &events {
        let t = Int::new_const(format!("t_{}", e.name));
        // constrain times to 0..1000
        solver.assert(&t.ge(&Int::from_i64(0)));
        solver.assert(&t.le(&Int::from_i64(1000)));
        event_times.insert(e.name.clone(), t);
    }

    // Collect all character names
    let mut characters: Vec<String> = chars.iter().map(|c| c.name.clone()).collect();

    // Map each character to an array of alive booleans for times 0..1000
    let mut alive_vars: HashMap<String, Vec<Bool>> = HashMap::new();
    for c in &characters {
        let mut alive_vec = Vec::new();
        for t in 0..=1000 {
            let b = Bool::new_const(format!("alive_{}_{}", c, t));
            alive_vec.push(b);
        }
        alive_vars.insert(c.clone(), alive_vec);
    }

    // Constraint 1: Event ordering
    for e in &events {
        let t1 = event_times.get(&e.name).unwrap();
        for b in &e.before {
            if let Some(t2) = event_times.get(b) {
                solver.assert(&t1.lt(t2));
            }
        }
    }



    // Constraint 2: Death effects
    for e in &events {
        let t = event_times.get(&e.name).unwrap();
        for eff in &e.effects {
            if let Effect::Death(c_name) = eff {
                let alive_vec = alive_vars.get(c_name).unwrap_or_else(|| {
                    eprintln!("❌ No entry found for key: {c_name:?}");
                    eprintln!("Current alive_vars keys: {:?}", alive_vars.keys());
                    panic!("Missing key in alive_vars: {}", c_name);
                });
                for time in 0..=1000 {
                    let time_int = Int::from_i64(time as i64);
                    // If time >= event_time => character dead
                    solver.assert(&time_int.ge(t).implies(&alive_vec[time].not()));
                }
            }
        }
    }

    // Constraint 3 & 4: Life propagation
    for c in &characters {
        let alive_vec = alive_vars.get(c).unwrap();
        // alive at t -> alive at t-1 (if t>0)
        for t in 1..=1000 {
            solver.assert(&alive_vec[t].implies(&alive_vec[t - 1]));
        }
        // not alive at t -> not alive at t+1 (if t<1000)
        for t in 0..1000 {
            solver.assert(&alive_vec[t].not().implies(&alive_vec[t + 1].not()));
        }
        // alive by default at time 0
        solver.assert(&alive_vec[0]);
    }

    for e in &events {
        let t = event_times.get(&e.name).unwrap();

        for (i, c) in e.characters.iter().enumerate() {
            let alive_vec = alive_vars.get(&c.name).unwrap();

            // find whether this event kills this character
            let is_death = e.effects.get(i).map(|eff| matches!(eff, Effect::Death(_))).unwrap_or(false);


            for time in 0..=1000 {
                let time_int = Int::from_i64(time as i64);

                if is_death {
                    // must be alive strictly *before* the event
                    solver.assert(&time_int.lt(t).implies(&alive_vec[time]));
                } else {
                    // must be alive *at* the event
                    solver.assert(&time_int._eq(t).implies(&alive_vec[time]));
                }
            }
        }
    }


    // Constraint 5: A character can only die once
    let mut death_events: HashMap<String, Vec<&Int>> = HashMap::new();

    for e in &events {
        let t = event_times.get(&e.name).unwrap();
        for eff in &e.effects {
            if let Effect::Death(c_name) = eff {
                death_events.entry(c_name.clone())
                    .or_default()
                    .push(t);
            }
        }
    }

    // For any two death events of the same character, enforce t1 == t2
    for (c, times) in death_events.clone()   {
        if times.len() > 1 {
            for i in 0..times.len() {
                for j in i + 1..times.len() {
                    // They must not both be distinct deaths
                    // Option 1: require equality (same moment of death)
                    // solver.assert(&times[i]._eq(times[j]));

                    // Option 2 (better): require distinct ordering impossible (t1 != t2)
                    // i.e. they can't both exist in the same valid model
                    solver.assert(&times[i]._eq(times[j]));
                }
            }
        }
    }

    for (c, times) in death_events {
        if times.len() > 1 {
            // Pick any two death events
            for i in 0..times.len() {
                for j in (i + 1)..times.len() {
                    // It's impossible for both events to "exist" for this character
                    // because that implies two separate Death(c) effects.
                    // Enforce: t_i != t_j is impossible if both are valid events.
                    // So add an UNSAT disjunction.
                    solver.assert(Bool::not(&times[i].eq(times[j]))); // <- use _ne, not _eq
                }
            }
        }
    }


    // Check satisfiability
    match solver.check() {
        SatResult::Sat => true,
        SatResult::Unsat => false,
        SatResult::Unknown => false, // conservatively return false on unknown
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Event, Character};
    use crate::types::Effect::Death;

    #[test]
    fn test_simple_sequence() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                track: 0.0,
                before: vec!["e2".to_string()],
                start: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec![],
                start: 0.0,
                track: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![],
            }
        ];
        assert!(isPossible(events, vec![Character{name: "Alice".to_string(), faction: "A".to_string()}]));
    }

    #[test]
    fn test_death_event() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                before: vec!["e2".to_string()],
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "combat".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec![Death("Bob".parse().unwrap())],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec![],
                start: 0.0,
                track: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec![],
            }
        ];
        // Bob dies in e1, but is in e2 -> impossible
        assert!(!isPossible(events, vec![Character{name: "Alice".to_string(), faction: "A".to_string()},
                                         Character{name: "Bob".to_string(), faction: "B".to_string()}]));
    }

    #[test]
    fn test_multiple_characters() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                before: vec!["e2".to_string()],
                start: 0.0,
                track: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![
                    Character { name: "Alice".to_string(), faction: "A".to_string() },
                    Character { name: "Bob".to_string(), faction: "B".to_string() },
                ],
                effects: vec![],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec![],
                start: 0.0,
                track: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![
                    Character { name: "Alice".to_string(), faction: "A".to_string() },
                ],
                effects: vec![],
            }
        ];
        assert!(isPossible(events, vec![Character{name: "Alice".to_string(), faction: "A".to_string()},
                                         Character{name: "Bob".to_string(), faction: "B".to_string()}]));
    }

    #[test]
    fn test_double_death_same_character() {
        let events = vec![
            Event {
                name: "death1".to_string(),
                description: "Bob dies the first time".to_string(),
                before: vec!["death2".to_string()],
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "catastrophe".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec![Death("Bob".to_string())],
            },
            Event {
                name: "death2".to_string(),
                description: "Bob dies again (impossible)".to_string(),
                before: vec![],
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "catastrophe".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec![Death("Bob".to_string())],
            }
        ];

        // Bob cannot die twice in a valid timeline
        assert!(
            !isPossible(
                events,
                vec![Character { name: "Bob".to_string(), faction: "B".to_string() }]
            ),
            "A character should not be able to die twice in sequence"
        );
    }


    #[test]
    fn test_chain_of_deaths() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                before: vec!["e2".to_string()],
                start: 0.0,
                track: 0.0,
                end: 0.0,
                _type: "combat".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![Death("Alice".to_string())],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec!["e3".to_string()],
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "combat".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec![Death("Bob".to_string())],
            },
            Event {
                name: "e3".to_string(),
                description: "".to_string(),
                before: vec![],
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Charlie".to_string(), faction: "C".to_string() }],
                effects: vec![],
            }
        ];
        // No contradictions, should be possible
        assert!(isPossible(events, vec![Character{name: "Alice".to_string(), faction: "A".to_string()},
                                         Character{name: "Bob".to_string(), faction: "B".to_string()},
                                        Character{name: "Charlie".to_string(), faction: "C".to_string()}]));
    }

    #[test]
    fn test_impossible_cycle() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                before: vec!["e2".to_string()],
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec!["e1".to_string()], // cycle
                start: 0.0,
                end: 0.0,
                track: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![],
            }
        ];
        assert!(!isPossible(events, vec![Character{name: "Alice".to_string(), faction: "A".to_string()}]));
    }
}
