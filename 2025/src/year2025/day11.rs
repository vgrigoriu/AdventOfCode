use color_eyre::{Result, eyre::eyre};
use std::{collections::{HashMap, HashSet, VecDeque}, str::FromStr};

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
    Err(eyre!("Not implemented yet."))
}

pub fn solve2() -> Result<usize> {
    let devices: Result<Vec<_>, _> = INPUT.lines().map(|l| l.parse::<Device>()).collect();
    let links: HashMap<_, _> = devices?.into_iter().map(|d|(d.name, d.outputs)).collect();

    let svr_to_fft = find_paths(&links, "svr", "fft");

    dbg!(svr_to_fft);
    Ok(svr_to_fft)
}

fn find_paths(links: &HashMap<String, Vec<String>>, from: &str, to: &str) -> usize {
    let mut to_visit: VecDeque<_> = vec![from].into();
    let mut visited = HashSet::new();

    let mut result = 0;
    let empty = vec![];
    while let Some(node) = to_visit.pop_front() {
        let links = links.get(node).unwrap_or(&empty);
        for link in links.iter() {
            if visited.contains(&(node, link)) {
                continue;
            }
            if link == to {
                println!("\t{node} outputs to {to}");
                result += 1;
            } else {
                to_visit.push_back(&link);
            }
            visited.insert((node, link));
        }
    }

    dbg!(result)
}

struct Device {
    name: String,
    outputs: Vec<String>,
}

impl FromStr for Device {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        // hhh: ccc fff iii
        let (name, outputs) = s.split_once(": ").ok_or(eyre!("No ':' in {}", s))?;
        let outputs = outputs.split(' ').map(|o| o.to_string()).collect();
        Ok(Self {
            name: name.to_string(),
            outputs,
        })
    }
}
