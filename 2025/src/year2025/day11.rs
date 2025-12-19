use crate::dag::DAG;
use color_eyre::Result;
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../../input/2025/day11.in");

pub fn solve1() -> Result<usize> {
    // Approach:
    // - compute number of incoming edges for every node
    // - set paths["svr"] = 1 (one way to reach start)
    // - process nodes in topological order:
    //   - add start node to a queue
    //   - while queue not empty:
    //     - pull node from queue
    //     - decrement incoming edges of neighbors
    //     - add paths[node] to paths[neighbor]
    //     - if incoming edges is 0, add neighbor to queue

    let graph = DAG::parse(INPUT)?;
    let mut incoming_edges = graph.incoming_edges();
    let mut to_visit: VecDeque<_> = incoming_edges.keys().filter(|&node|incoming_edges[node] == 0).copied().collect();

    // walk until `you`
    while let Some(node) = to_visit.pop_front() {
        if node == "you" {
            break;
        }
        for &neighbor in graph.meighbors(node) {
            let incoming_edges_entry = incoming_edges.entry(neighbor).or_default();
            *incoming_edges_entry -= 1;
            if *incoming_edges_entry == 0 {
                to_visit.push_back(neighbor);
            }
        }
    }
    let mut paths = HashMap::new();
    paths.insert("you", 1);
    //to_visit = VecDeque::new();
    to_visit.push_back("you");

    // walk until `out`
    while let Some(node) = to_visit.pop_front() {
        if node == "out" {
            break;
        }
        for &neighbor in graph.meighbors(node) {
            let incoming_edges_entry = incoming_edges.entry(neighbor).or_default();
            *incoming_edges_entry -= 1;
            *paths.entry(neighbor).or_default() += *paths.entry(node).or_default();
            if *incoming_edges_entry == 0 {
                to_visit.push_back(neighbor);
            }
        }
    }
    Ok(paths["out"])
}

pub fn solve2() -> Result<usize> {
    let graph = DAG::parse(INPUT)?;
    let mut incoming_edges = graph.incoming_edges();
    let mut to_visit: VecDeque<_> = incoming_edges.keys().filter(|&node|incoming_edges[node] == 0).copied().collect();

    // count paths from `svr` to `fft`
    let mut paths = HashMap::new();
    paths.insert("svr", 1);

    // walk until `fft`
    while let Some(node) = to_visit.pop_front() {
        if node == "fft" {
            break;
        }
        for &neighbor in graph.meighbors(node) {
            let incoming_edges_entry = incoming_edges.entry(neighbor).or_default();
            *incoming_edges_entry -= 1;
            *paths.entry(neighbor).or_default() += *paths.entry(node).or_default();
            if *incoming_edges_entry == 0 {
                to_visit.push_back(neighbor);
            }
        }
    }
    let paths_svr_to_fft = paths["fft"];

    // count paths from `fft` to `dac`
    paths.clear();
    paths.insert("fft", 1);
    to_visit.push_back("fft");

    // walk until `dac`
    while let Some(node) = to_visit.pop_front() {
        if node == "dac" {
            break;
        }
        for &neighbor in graph.meighbors(node) {
            let incoming_edges_entry = incoming_edges.entry(neighbor).or_default();
            *incoming_edges_entry -= 1;
            *paths.entry(neighbor).or_default() += *paths.entry(node).or_default();
            if *incoming_edges_entry == 0 {
                to_visit.push_back(neighbor);
            }
        }
    }
    let paths_fft_to_dac = paths["dac"];

    // count paths from `dac` to `out`
    paths.clear();
    paths.insert("dac", 1);
    to_visit.push_back("dac");

    // walk until `out`
    while let Some(node) = to_visit.pop_front() {
        if node == "out" {
            break;
        }
        for &neighbor in graph.meighbors(node) {
            let incoming_edges_entry = incoming_edges.entry(neighbor).or_default();
            *incoming_edges_entry -= 1;
            *paths.entry(neighbor).or_default() += *paths.entry(node).or_default();
            if *incoming_edges_entry == 0 {
                to_visit.push_back(neighbor);
            }
        }
    }
    let paths_dac_to_out = paths["out"];

    dbg!(paths_svr_to_fft);
    dbg!(paths_fft_to_dac);
    dbg!(paths_dac_to_out);
    Ok(paths_svr_to_fft * paths_fft_to_dac * paths_dac_to_out)
}
