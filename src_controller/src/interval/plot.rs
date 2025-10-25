use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::{is_cyclic_directed, toposort};
use plotters::prelude::*;
use std::collections::HashMap;

/// Add a new constraint, validate, compute its interval, and save graph to PNG
pub fn add_constraint_and_get_interval(
    existing_constraints: Vec<(&str, &str)>,
    new_constraint: (&str, &str),
    output_file: &str,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    // --- Build DAG ---------------------------------------------------------
    let mut graph = DiGraph::<&str, ()>::new();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

    for &(a, b) in &existing_constraints {
        let a_idx = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
        let b_idx = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(a_idx, b_idx, ());
    }

    let (a, b) = new_constraint;
    let a_idx = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
    let b_idx = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
    graph.add_edge(a_idx, b_idx, ());

    // --- Validate interval graph ------------------------------------------
    if !is_interval_graph(&graph) {
        return Err("Adding this constraint breaks interval graph properties".into());
    }

    // --- Topological sort -------------------------------------------------
    let order = toposort(&graph, None).map_err(|_| "Graph has cycles")?;

    // --- Compute earliest times -------------------------------------------
    let mut earliest: HashMap<NodeIndex, f64> = HashMap::new();
    for &n in &order {
        let preds: Vec<_> = graph.neighbors_directed(n, petgraph::Incoming).collect();
        let max_pred = preds
            .iter()
            .map(|p| earliest.get(p).unwrap_or(&0.0))
            .cloned()
            .fold(0.0, f64::max);
        earliest.insert(n, max_pred + 1.0);
    }

    // --- Compute latest times ---------------------------------------------
    let mut latest: HashMap<NodeIndex, f64> = HashMap::new();
    let max_earliest = *earliest.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    for &n in order.iter().rev() {
        let succs: Vec<_> = graph.neighbors_directed(n, petgraph::Outgoing).collect();
        let min_succ = succs
            .iter()
            .map(|s| *latest.get(s).unwrap_or(&(max_earliest + 1.0)))
            .fold(f64::INFINITY, f64::min);
        latest.insert(n, if min_succ.is_infinite() { max_earliest + 1.0 } else { min_succ - 1.0 });
    }

    // --- Normalize intervals ----------------------------------------------
    let min_e = *earliest.values().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_l = *latest.values().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let total_span = max_l - min_e;

    let mut segments: HashMap<&str, (f64, f64)> = HashMap::new();
    for (&node_idx, &start_val) in &earliest {
        let start = (start_val - min_e) / total_span;
        let end_val = *latest.get(&node_idx).unwrap();
        let mut end = (end_val - min_e) / total_span;
        if end < start { end = start + 0.05; }
        segments.insert(graph[node_idx], (start, end));
    }

    // --- Plot graph to PNG -------------------------------------------------
    let root = BitMapBackend::new(output_file, (800, (200 + 50 * segments.len()).try_into().unwrap())).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(20)
        .build_cartesian_2d(0f64..1f64, 0f64..(segments.len() as f64))?;

    chart.configure_mesh().disable_mesh().draw()?;

    for (i, (name, (start, end))) in segments.iter().enumerate() {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(start.clone(), i as f64), (end.clone(), i as f64 + 0.8)],
            RGBColor(0, 100 + (i as u8 * 20 % 155), 200).filled(),
        )))?;

        chart.draw_series(std::iter::once(Text::new(
            (*name).to_string(),
            ((start + end)/2.0, i as f64 + 0.4),
            ("sans-serif", 15).into_font().color(&BLACK),
        )))?;
    }

    // --- Return interval for new constraint --------------------------------
    let start_new = (earliest[&a_idx] - min_e) / total_span;
    let end_new = (latest[&b_idx] - min_e) / total_span;

    Ok((start_new, end_new))
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

        for neighbor in graph.neighbors(node).chain(graph.neighbors_directed(node, petgraph::Incoming)) {
            if !visited[neighbor.index()] {
                stack.push(neighbor);
            }
        }
    }

    visited.into_iter().all(|v| v)
}
