use z3::{Config, Context, Solver, ast::{Int, Bool}, SatResult};
use crate::types::{Event, Character};
use std::collections::HashMap;

pub fn isPossible(events: Vec<Event>) -> bool {
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
    let mut characters: Vec<String> = events.iter()
        .flat_map(|e| e.characters.iter().map(|c| c.name.clone()))
        .collect();
    characters.sort();
    characters.dedup();

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
        for (i, eff) in e.effects.iter().enumerate() {
            if eff == "death" {
                let c_name = &e.characters[i].name;
                let alive_vec = alive_vars.get(c_name).unwrap();
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
            let is_death = e.effects.get(i).map(|s| s == "death").unwrap_or(false);

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

    #[test]
    fn test_simple_sequence() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
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
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![],
            }
        ];
        assert!(isPossible(events));
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
                _type: "combat".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec!["death".to_string()],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec![],
                start: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec![],
            }
        ];
        // Bob dies in e1, but is in e2 -> impossible
        assert!(!isPossible(events));
    }

    #[test]
    fn test_multiple_characters() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                before: vec!["e2".to_string()],
                start: 0.0,
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
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![
                    Character { name: "Alice".to_string(), faction: "A".to_string() },
                ],
                effects: vec![],
            }
        ];
        assert!(isPossible(events));
    }

    #[test]
    fn test_chain_of_deaths() {
        let events = vec![
            Event {
                name: "e1".to_string(),
                description: "".to_string(),
                before: vec!["e2".to_string()],
                start: 0.0,
                end: 0.0,
                _type: "combat".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec!["death".to_string()],
            },
            Event {
                name: "e2".to_string(),
                description: "".to_string(),
                before: vec!["e3".to_string()],
                start: 0.0,
                end: 0.0,
                _type: "combat".to_string(),
                characters: vec![Character { name: "Bob".to_string(), faction: "B".to_string() }],
                effects: vec!["death".to_string()],
            },
            Event {
                name: "e3".to_string(),
                description: "".to_string(),
                before: vec![],
                start: 0.0,
                end: 0.0,
                _type: "normal".to_string(),
                characters: vec![Character { name: "Charlie".to_string(), faction: "C".to_string() }],
                effects: vec![],
            }
        ];
        // No contradictions, should be possible
        assert!(isPossible(events));
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
                _type: "normal".to_string(),
                characters: vec![Character { name: "Alice".to_string(), faction: "A".to_string() }],
                effects: vec![],
            }
        ];
        assert!(!isPossible(events));
    }
}
