use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../../input/day08.in");

#[derive(Debug, Copy, Clone)]
struct JunctionBox {
    no: usize,
    x: f64,
    y: f64,
    z: f64,
}

impl JunctionBox {
    fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y) + (self.z - other.z) * (self.z - other.z)).sqrt()
    }
}

#[derive(Debug)]
struct Pair {
    box1: JunctionBox,
    box2: JunctionBox,
    distance: f64
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.total_cmp(&other.distance))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.box1.no == other.box1.no && self.box2.no == other.box2.no
    }
}

impl Eq for Pair {}

pub fn solve1() {
    let boxes: Vec<_> = INPUT.lines().enumerate().map(|(i, line)|{
        let parts: Vec<_> = line.split(",").map(|part| part.parse().unwrap()).collect();
        JunctionBox {no: i, x: parts[0], y: parts[1], z: parts[2]}
    }).collect();

    let mut all_pairs = vec![];
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            all_pairs.push(Pair{box1: boxes[i], box2: boxes[j], distance: boxes[i].distance_to(&boxes[j])});
        }
    }

    all_pairs.sort();

    let mut circuits: HashMap<usize, _> = HashMap::new();
    let mut circuit_no = 0;
    for pair in &all_pairs[..1000] {
        if !circuits.contains_key(&pair.box1.no) && !circuits.contains_key(&pair.box2.no) {
            // No box is in a circuit, create a new one.
            circuits.insert(pair.box1.no, circuit_no);
            circuits.insert(pair.box2.no, circuit_no);
            circuit_no += 1;
        } else if circuits.contains_key(&pair.box1.no) && !circuits.contains_key(&pair.box2.no) {
            // Add box2 into box1's circuit.
            circuits.insert(pair.box2.no, *circuits.get(&pair.box1.no).unwrap());
        } else if !circuits.contains_key(&pair.box1.no) && circuits.contains_key(&pair.box2.no) {
            // Add box1 into box2's circuit.
            circuits.insert(pair.box1.no, *circuits.get(&pair.box2.no).unwrap());
        } else {
            // Both are in circuits, add the boxes in the second circuit to the first circuit,
            // unless they already are in the same circuit.
            let circuit1 = *circuits.get(&pair.box1.no).unwrap();
            let circuit2 = *circuits.get(&pair.box2.no).unwrap();
            if circuit1 == circuit2 {
                continue;
            }
            for (box_no, &circuit) in circuits.clone().iter() {
                if circuit == circuit2 {
                    *circuits.get_mut(&box_no).unwrap() = circuit1;
                }
            }
        }
    }

    let mut size_of_circuits = HashMap::new();
    for (_, circuit) in circuits {
        if !size_of_circuits.contains_key(&circuit) {
            size_of_circuits.insert(circuit, 1);
        } else {
            *size_of_circuits.get_mut(&circuit).unwrap() += 1;
        }
    }

    for (circuit, size) in size_of_circuits {
        println!("{circuit}: {size}");
    }

    println!("{}", 55 * 34 * 31);
}

pub fn solve2() {
        let boxes: Vec<_> = INPUT.lines().enumerate().map(|(i, line)|{
        let parts: Vec<_> = line.split(",").map(|part| part.parse().unwrap()).collect();
        JunctionBox {no: i, x: parts[0], y: parts[1], z: parts[2]}
    }).collect();

    let mut all_pairs = vec![];
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            all_pairs.push(Pair{box1: boxes[i], box2: boxes[j], distance: boxes[i].distance_to(&boxes[j])});
        }
    }

    all_pairs.sort();

    let mut circuits: HashMap<usize, _> = HashMap::new();
    let mut circuit_no = 0;
    for pair in &all_pairs {
        if !circuits.contains_key(&pair.box1.no) && !circuits.contains_key(&pair.box2.no) {
            // No box is in a circuit, create a new one.
            circuits.insert(pair.box1.no, circuit_no);
            circuits.insert(pair.box2.no, circuit_no);
            circuit_no += 1;
        } else if circuits.contains_key(&pair.box1.no) && !circuits.contains_key(&pair.box2.no) {
            // Add box2 into box1's circuit.
            circuits.insert(pair.box2.no, *circuits.get(&pair.box1.no).unwrap());
        } else if !circuits.contains_key(&pair.box1.no) && circuits.contains_key(&pair.box2.no) {
            // Add box1 into box2's circuit.
            circuits.insert(pair.box1.no, *circuits.get(&pair.box2.no).unwrap());
        } else {
            // Both are in circuits, add the boxes in the second circuit to the first circuit,
            // unless they already are in the same circuit.
            let circuit1 = *circuits.get(&pair.box1.no).unwrap();
            let circuit2 = *circuits.get(&pair.box2.no).unwrap();
            if circuit1 == circuit2 {
                continue;
            }
            for (box_no, &circuit) in circuits.clone().iter() {
                if circuit == circuit2 {
                    *circuits.get_mut(&box_no).unwrap() = circuit1;
                }
            }
        }

        if circuits.len() == boxes.len() {
            println!("{}", pair.box1.x * pair.box2.x);
            return;
        }
    }


}