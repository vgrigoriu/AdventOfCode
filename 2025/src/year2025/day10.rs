use crate::solver::solve_system_minimizing_sum;
use regex::Regex;

const INPUT: &str = include_str!("../../input/2025/day10.in");

pub fn solve1() {
    let machines: Vec<_> = INPUT.lines().map(Machine::parse).collect();

    let min_presses: u32 = machines.iter().map(|m| m.least_button_presses()).sum();

    println!("{min_presses}");
}

pub fn solve2() {
    let machines: Vec<_> = INPUT.lines().map(Machine::parse).collect();

    let solutions: Vec<_> = machines
        .iter()
        .map(|m| solve_system_minimizing_sum(&m.get_coefficients(), &m.joltages).unwrap())
        .collect();

    // check solutions
    for (machine, solution) in machines.iter().zip(solutions.iter()) {
        //println!("Machine: {}\nSolution: {:?}", machine.input, solution);
        machine.check_solution(&solution);
    }

    let solution: u32 = solutions.iter().map(|s| s.iter().sum::<u32>()).sum();
    println!("{solution}");
}

// [.####] (0,1,4) (0,1,2) (0,1,2,4) (1,2,3) {26,38,36,12,22}

#[derive(Debug)]
struct Button {
    toggles: Vec<usize>,
}

impl Button {
    fn parse(line: &str) -> Self {
        // `(0,3,4,5,6)`
        let toggles: Vec<usize> = line[1..line.len() - 1]
            .split(",")
            .filter_map(|s| s.parse().ok())
            .collect();
        Self { toggles }
    }
}

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
    input: String,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let re =
            Regex::new(r"^\[(?<lights>.+)\] (?<buttons>(\([^)]+\) )+)\{(?<joltages>.+)\}").unwrap();
        let captures = re.captures(line).unwrap();

        let target_lights: Vec<_> = captures["lights"].chars().map(|ch| ch == '#').collect();
        let buttons: Vec<_> = captures["buttons"]
            .split(" ")
            .filter(|&s| !s.is_empty())
            .map(Button::parse)
            .collect();
        let joltages = captures["joltages"]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            target_lights,
            buttons,
            joltages,
            input: line.to_string(),
        }
    }

    fn least_button_presses(&self) -> u32 {
        let mut min_presses = u32::MAX;
        let combinations = 2_u32.pow(self.buttons.len() as u32);
        for combination in 0..combinations {
            let mut lights = vec![false; self.target_lights.len()];
            for bit in 0..self.buttons.len() {
                if combination & (1 << bit) != 0 {
                    Self::push_button(&mut lights, &self.buttons[bit]);
                }
            }

            if lights == self.target_lights {
                if combination.count_ones() < min_presses {
                    min_presses = combination.count_ones();
                }
            }
        }

        min_presses
    }

    fn push_button(lights: &mut Vec<bool>, button: &Button) {
        for &toggle in &button.toggles {
            lights[toggle] = !lights[toggle];
        }
    }

    pub fn get_coefficients(&self) -> Vec<Vec<u32>> {
        // If we have the following buttons: (3) (1,3) (2) (2,3) (0,2) (0,1)
        // then we get the following coefficients:
        // [0, 0, 0, 0, 1, 1]
        // [0, 1, 0, 0, 0, 1]
        // [0, 0, 1, 1, 1, 0]
        // [1, 1, 0, 1, 0, 0]
        // Each column corresponds to a button, and the coefficient is 1
        // if the row index appears in the button toggles.

        // number of equations = number of joltages
        // number of variables = number of buttons
        let mut equations = vec![vec![0; self.buttons.len()]; self.joltages.len()];
        for (rank, button) in self.buttons.iter().enumerate() {
            for &toggle in &button.toggles {
                equations[toggle][rank] = 1;
            }
        }

        equations
    }

    pub fn check_solution(&self, presses: &[u32]) {
        let mut joltages = vec![0; self.joltages.len()];

        for (button, &times) in self.buttons.iter().zip(presses) {
            for &toggle in &button.toggles {
                joltages[toggle] += times;
            }
        }

        if self.joltages != joltages {
            let diff: Vec<_> = self
                .joltages
                .iter()
                .zip(joltages.iter())
                .map(|(e, a)| e - a)
                .collect();
            let extra_solution = solve_system_minimizing_sum(&self.get_coefficients(), &diff);
            println!(
                "{}\n\tSolution: {:?}\n\tDiff: {:?}\n\tExtra solution: {:?}",
                self.input,
                presses,
                diff,
                extra_solution.unwrap()
            );
        }
    }
}
