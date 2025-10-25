use crate::types::{Map, Place};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use serde::{Serialize, Deserialize};

impl Serialize for Place {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.name, &self.location).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Place {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (name, location) = <(String, (f64, f64))>::deserialize(deserializer)?;
        Ok(Place { name, location })
    }
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.locations, &self.routes).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (locations, routes) = <(Vec<Place>, Vec<Vec<(f64, f64)>>)>::deserialize(deserializer)?;
        Ok(Map { locations, routes })
    }
}

/// Write a Map to a JSON file
pub fn write_map_to_file(map: &Map, path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, map)?;
    Ok(())
}

/// Read a Map from a JSON file
pub fn read_map_from_file(path: &str) -> io::Result<Map> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}
