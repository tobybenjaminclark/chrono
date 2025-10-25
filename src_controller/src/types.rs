use std::collections::HashMap;
use std::fmt;

use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Place {
    pub name: String,
    pub location: (f64, f64),
}

impl PartialEq for Place {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.location == other.location
    }
}

impl Eq for Place {}

impl Hash for Place {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.location.0.to_bits().hash(state); // f64 cannot be hashed directly
        self.location.1.to_bits().hash(state);
    }
}


impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:.5}, {:.5})", self.name, self.location.0, self.location.1)
    }
}

#[derive(Clone)]
pub struct Map {
    pub locations: Vec<Place>,
    pub routes: Vec<Vec<(f64, f64)>>
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Locations:")?;
        for (i, place) in self.locations.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, place)?;
        }
        writeln!(f, "Routes: {}", self.routes.len())?;
        for (i, rout qe) in self.routes.iter().enumerate() {
            // Take first 5 points or fewer
            let first_points: Vec<String> = route.iter()
                .take(5)
                .map(|(x, y)| format!("({:.5}, {:.5})", x, y))
                .collect();
            writeln!(
                f,
                "  Route {} ({} points, first 5: [{}])",
                i + 1,
                route.len(),
                first_points.join(", ")
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Faction {
    #[serde(rename = "g")]
    Gnomes,
    #[serde(rename = "t")]
    Trolls,
    #[serde(rename = "c")]
    Centaurs,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Event {
    pub ownership: HashMap<Place, Faction>,
}


#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct Character {
    pub(crate) name: String,
    pub(crate) faction: Faction,
}

pub fn ownership_to_json_map(event: &Event) -> HashMap<String, Faction> {
    let mut map = HashMap::new();
    for (place, faction) in &event.ownership {
        map.insert(place.clone().name, *faction);
    }
    map
}
