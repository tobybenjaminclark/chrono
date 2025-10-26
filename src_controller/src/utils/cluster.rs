use std::cmp::PartialEq;
use crate::types::{Event, Map, Ownership, Place};
use std::collections::HashMap;
use rand::prelude::IndexedRandom;

/// Compute squared Euclidean distance between two points
fn dist2(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx * dx + dy * dy
}
pub fn cluster_locations(map: &Map) -> Ownership {
    let factions = ["g", "t", "c"];
    let k = factions.len();
    let max_per_cluster = (map.locations.len() as f64 / k as f64).ceil() as usize;

    // Step 1: Initialize centroids randomly
    let mut centroids: Vec<(f64, f64)> = map.locations
        .iter()
        .take(k)
        .map(|p| p.location)
        .collect();

    let mut ownership: HashMap<String, String> = HashMap::new();
    let mut changed = true;
    let mut iterations = 0;
    let max_iterations = 100;

    while changed && iterations < max_iterations {
        iterations += 1;
        changed = false;

        // Step 2: Assign points to closest centroid respecting max_per_cluster
        let mut clusters: Vec<Vec<&str>> = vec![Vec::new(); k]; // store place names
        let mut cluster_counts = vec![0; k];

        for place in &map.locations {
            // compute distances to centroids
            let mut distances: Vec<(usize, f64)> = centroids.iter()
                .enumerate()
                .map(|(i, &(cx, cy))| (i, ((place.location.0 - cx).powi(2) + (place.location.1 - cy).powi(2)).sqrt()))
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            // assign to the closest centroid with available capacity
            for &(idx, _) in &distances {
                if cluster_counts[idx] < max_per_cluster {
                    clusters[idx].push(&place.name);
                    cluster_counts[idx] += 1;
                    break;
                }
            }
        }

        // Step 3: Recompute centroids
        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.is_empty() { continue; }
            let sum = cluster.iter().fold((0.0, 0.0), |acc, name| {
                let p = map.locations.iter().find(|pl| &pl.name == *name).unwrap();
                (acc.0 + p.location.0, acc.1 + p.location.1)
            });
            centroids[i] = (sum.0 / cluster.len() as f64, sum.1 / cluster.len() as f64);
        }

        // Step 4: Update ownership
        for (i, cluster) in clusters.iter().enumerate() {
            for &name in cluster {
                let faction = factions[i];
                if ownership.get(name) != Some(&faction.to_string()) {
                    ownership.insert(name.to_string(), faction.to_string());
                    changed = true;
                }
            }
        }
    }

    // Step 5: Map ownership back to Place
    let mut final_ownership = HashMap::new();
    for place in &map.locations {
        if let Some(faction) = ownership.get(&place.name) {
            final_ownership.insert(place.clone(), faction.clone());
        }
    }
    final_ownership
}
