use crate::dag::DAG;
use color_eyre::Result;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../../input/2025/day11.in");

pub fn solve1() -> Result<usize> {
    let path_finder = PathsFinder::new(DAG::parse(INPUT)?);
    let result = path_finder.find_paths(&["you", "out"]);
    Ok(result)
}

pub fn solve2() -> Result<usize> {
    let path_finder = PathsFinder::new(DAG::parse(INPUT)?);
    let result = path_finder.find_paths(&["svr", "fft", "dac", "out"]);
    Ok(result)
}

struct PathsFinder<'a> {
    dag: DAG<'a>,
}

impl<'a> PathsFinder<'a> {
    pub fn new(dag: DAG<'a>) -> Self {
        Self { dag }
    }

    pub fn find_paths(&self, nodes: &[&str]) -> usize {
        // Approach:
        // - compute number of incoming edges for every node
        // - set paths[start node] = 1 (one way to reach start)
        // - process nodes in topological order:
        //   - add start node to a queue
        //   - while queue not empty:
        //     - pull node from queue
        //     - decrement incoming edges of neighbors
        //     - add paths[node] to paths[neighbor]
        //     - if incoming edges is 0, add neighbor to queue

        let mut incoming_edges = self.dag.incoming_edges();
        // Start by visiting root nodes.
        let mut to_visit: VecDeque<_> = incoming_edges
            .keys()
            .filter(|&node| incoming_edges[node] == 0)
            .copied()
            .collect();

        // if first node is not a root node, walk from the first root until it
        // to remove incoming edges, and ignore the result.
        if !to_visit.contains(&nodes[0]) {
            let _ = self.walk(to_visit[0], nodes[0], &mut incoming_edges, &mut to_visit);
        }

        let mut result = 1;
        for (node1, node2) in nodes.iter().tuple_windows() {
            result *= self.walk(node1, node2, &mut incoming_edges, &mut to_visit);
        }

        result
    }

    fn walk(
        &self,
        from: &'a str,
        to: &str,
        incoming_edges: &mut HashMap<&'a str, usize>,
        to_visit: &mut VecDeque<&'a str>,
    ) -> usize {
        let mut paths = HashMap::new();
        paths.insert(from, 1);

        if !to_visit.contains(&from) {
            to_visit.push_back(from);
        }

        // walk until `to`
        while let Some(node) = to_visit.pop_front() {
            if node == to {
                break;
            }
            for &neighbor in self.dag.neighbors(node) {
                let incoming_edges_entry = incoming_edges.entry(neighbor).or_default();
                *incoming_edges_entry -= 1;
                *paths.entry(neighbor).or_default() += *paths.entry(node).or_default();
                if *incoming_edges_entry == 0 {
                    to_visit.push_back(neighbor);
                }
            }
        }

        paths[to]
    }
}
