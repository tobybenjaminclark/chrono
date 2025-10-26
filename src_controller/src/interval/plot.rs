use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{is_cyclic_directed, toposort};
use plotters::prelude::*;
use std::collections::HashMap;

use crate::types::Event;

/// Add a new constraint (a must happen before b), validate feasibility,
/// compute normalized intervals for all events, update their start/end times,
/// and save a timeline visualization as a PNG.
///
/// Returns ((start, end) for the new constraint, updated events)
pub fn add_constraint_and_get_interval(
    mut existing_events: Vec<Event>,
    new_constraint: (&str, &str),
    output_file: &str,
) -> Result<((f64, f64), i32, Vec<Event>), Box<dyn std::error::Error>> {
    let (a, b) = new_constraint;

    // --- Update "before" list for event `a` ----------------------------
    if let Some(idx) = existing_events.iter().position(|e| e.name == a) {
        let ev = &mut existing_events[idx];
        if !ev.before.contains(&b.to_string()) {
            ev.before.push(b.to_string());
        }
    }

    // --- Build DAG -----------------------------------------------------
    let mut graph = DiGraph::<String, ()>::new();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    for event in &existing_events {
        let a_idx = *nodes.entry(event.name.clone())
            .or_insert_with(|| graph.add_node(event.name.clone()));
        for b_name in &event.before {
            let b_idx = *nodes.entry(b_name.clone())
                .or_insert_with(|| graph.add_node(b_name.clone()));
            graph.add_edge(a_idx, b_idx, ());
        }
    }

    // --- Add new constraint --------------------------------------------
    let a_idx = *nodes.entry(a.to_string()).or_insert_with(|| graph.add_node(a.to_string()));
    let b_idx = *nodes.entry(b.to_string()).or_insert_with(|| graph.add_node(b.to_string()));
    if !graph.contains_edge(a_idx, b_idx) {
        graph.add_edge(a_idx, b_idx, ());
    }

    // --- Validate DAG --------------------------------------------------
    if is_cyclic_directed(&graph) {
        return Err("Adding this constraint introduces a cycle".into());
    }
    if !is_interval_graph(&graph) {
        return Err("Graph is not a valid interval graph".into());
    }

    // --- Topological sort ---------------------------------------------
    let order = toposort(&graph, None).map_err(|_| "Graph has cycles")?;

    // --- Compute earliest ---------------------------------------------
    let mut earliest: HashMap<NodeIndex, f64> = HashMap::new();
    for &n in &order {
        let preds: Vec<_> = graph.neighbors_directed(n, petgraph::Incoming).collect();
        let max_pred = preds
            .iter()
            .map(|p| *earliest.get(p).unwrap_or(&0.0))
            .fold(0.0, f64::max);
        earliest.insert(n, max_pred + 1.0);
    }

    // --- Compute latest -----------------------------------------------
    let mut latest: HashMap<NodeIndex, f64> = HashMap::new();
    let max_earliest = *earliest.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    for &n in order.iter().rev() {
        let succs: Vec<_> = graph.neighbors_directed(n, petgraph::Outgoing).collect();
        let min_succ_earliest = succs
            .iter()
            .map(|s| *earliest.get(s).unwrap_or(&(max_earliest + 1.0)))
            .fold(f64::INFINITY, f64::min);

        latest.insert(n, if min_succ_earliest.is_infinite() {
            max_earliest + 1.0
        } else {
            min_succ_earliest
        });
    }


    // --- Normalize intervals ------------------------------------------
    let min_e = *earliest.values().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_l = *latest.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let total_span = if (max_l - min_e).abs() < f64::EPSILON { 1.0 } else { max_l - min_e };

    let mut segments: HashMap<String, (f64, f64)> = HashMap::new();
    for (&node_idx, &start_val) in &earliest {
        let start = (start_val - min_e) / total_span;
        let end_val = *latest.get(&node_idx).unwrap();
        let mut end = (end_val - min_e) / total_span;
        if end < start { end = start + 0.05; }
        segments.insert(graph[node_idx].clone(), (start, end));
    }

    // --- Update event intervals ---------------------------------------
    for event in &mut existing_events {
        if let Some((s, e)) = segments.get(&event.name) {
            event.start = *s;
            event.end = *e;
        }
    }

    // --- Assign tracks and determine new constraint track -------------
    let mut tracks: Vec<f64> = Vec::new(); // stores end of last event on each track
    let mut new_constraint_track: f32 = -1.0;

    for event in &mut existing_events {
        let (start, end) = segments[&event.name];
        let mut assigned = false;

        for (track_idx, last_end) in tracks.iter_mut().enumerate() {
            if *last_end <= start {
                // Can fit on this track
                event.track = track_idx as f32;
                *last_end = end;
                assigned = true;
                break;
            }
        }

        if !assigned {
            // New track needed
            tracks.push(end);
            event.track = (tracks.len() - 1) as f32;
        }

        if event.name == a || event.name == b {
            new_constraint_track = new_constraint_track.max(event.track);
        }
    }


    // --- Plot to PNG ---------------------------------------------------
    let height = 200 + 50 * tracks.len();
    let root = BitMapBackend::new(output_file, (800, height as u32)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(20)
        .build_cartesian_2d(0f64..1f64, 0f64..(tracks.len() as f64))?;

    chart.configure_mesh().disable_mesh().draw()?;

    for event in &existing_events {
        let track = event.track as f64;
        let (start, end) = segments[&event.name];

        chart.draw_series(std::iter::once(Rectangle::new(
            [(start, track), (end, track + 0.8)],
            RGBColor(0, 100 + ((track as u8 * 30) % 155), 200).filled(),
        )))?;

        chart.draw_series(std::iter::once(Text::new(
            event.name.clone(),
            ((start + end) / 2.0, track + 0.4),
            ("sans-serif", 15).into_font().color(&BLACK),
        )))?;
    }

    // --- Return interval for new constraint ---------------------------
    let start_new = (earliest[&a_idx] - min_e) / total_span;
    let end_new = (earliest[&b_idx] - min_e) / total_span; // <-- use earliest here


    Ok(((start_new, end_new), new_constraint_track as i32, existing_events))
}



/// Returns true if the graph is weakly connected and acyclic
pub fn is_interval_graph<N>(graph: &DiGraph<N, ()>) -> bool {
    if is_cyclic_directed(graph) {
        return false;
    }

    if graph.node_count() == 0 {
        return true;
    }

    let mut visited = vec![false; graph.node_count()];
    let start = NodeIndex::new(0);
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if visited[node.index()] {
            continue;
        }
        visited[node.index()] = true;

        for neighbor in graph.neighbors(node)
            .chain(graph.neighbors_directed(node, petgraph::Incoming)) {
            if !visited[neighbor.index()] {
                stack.push(neighbor);
            }
        }
    }

    visited.into_iter().all(|v| v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn test_add_constraint_simple() {
        let mut events = vec![
            Event { name: "A".to_string(), description: "".to_string(), before: vec![], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
            Event { name: "B".to_string(), description: "".to_string(), before: vec![], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
        ];

        let dir = tempdir().unwrap();
        let output_file = dir.path().join("timeline.png").to_str().unwrap().to_string();

        let ((start, end), _, updated_events) = add_constraint_and_get_interval(
            events.clone(),
            ("A", "B"),
            &output_file
        ).unwrap();

        // Check that the new constraint interval is valid
        assert!(start < end, "Start must be less than end for new constraint");

        // Check that the "before" list in event A is updated
        let event_a = updated_events.iter().find(|e| e.name == "A").unwrap();
        assert!(event_a.before.contains(&"B".to_string()));

        // Check that start/end times are normalized between 0 and 1
        for e in &updated_events {
            assert!(e.start >= 0.0 && e.start <= 1.0);
            assert!(e.end >= 0.0 && e.end <= 1.0);
            assert!(e.start <= e.end);
        }
    }

    #[test]
    fn test_cycle_detection() {
        let events = vec![
            Event { name: "X".to_string(), description: "".to_string(), before: vec!["Y".to_string()], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
            Event { name: "Y".to_string(), description: "".to_string(), before: vec![], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
        ];

        let dir = tempdir().unwrap();
        let output_file = dir.path().join("timeline.png").to_str().unwrap().to_string();

        // Adding a constraint Y -> X should create a cycle
        let result = add_constraint_and_get_interval(events, ("Y", "X"), &output_file);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cycle"));
    }

    #[test]
    fn test_no_cycle_multiple_events() {
        let mut events = vec![
            Event { name: "A".to_string(), description: "".to_string(), before: vec!["B".to_string()], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
            Event { name: "B".to_string(), description: "".to_string(), before: vec!["C".to_string()], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
            Event { name: "C".to_string(), description: "".to_string(), before: vec![], start: 0.0, end: 0.0, _type: "".to_string(), characters: vec![], effects: vec![], track: 0.0 },
        ];

        let dir = tempdir().unwrap();
        let output_file = dir.path().join("timeline.png").to_str().unwrap().to_string();

        // Adding a new constraint A -> C is fine
        let result = add_constraint_and_get_interval(events.clone(), ("A", "C"), &output_file);
        assert!(result.is_ok());
        let (_, _, updated_events) = result.unwrap();

        // Check that the "before" lists were updated properly
        let a_event = updated_events.iter().find(|e| e.name == "A").unwrap();
        assert!(a_event.before.contains(&"C".to_string()));
    }
}

