use std::u8;

use regex::Regex;

const INPUT: &str = include_str!("../input/day10.in");

pub fn solve1() {
    let machines: Vec<_> = INPUT.lines().map(Machine::parse).collect();

    let min_presses:u32 = machines.iter().map(|m| m.least_button_presses()).sum();

    println!("{min_presses}");
}

pub fn solve2() {
    println!("{}", INPUT.lines().count());
}

// [.####] (0,1,4) (0,1,2) (0,1,2,4) (1,2,3) {26,38,36,12,22}

#[derive(Debug)]
struct Button {
    toggles: Vec<usize>
}

impl Button {
    fn parse(line: &str) -> Self {
        // `(0,3,4,5,6)`
        let toggles: Vec<usize> = line[1..line.len()-1].split(",").filter_map(|s| s.parse().ok()).collect();
        Self {toggles}
    }
}

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Button>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let re = Regex::new(r"^\[(?<lights>.+)\] (?<buttons>(\([^)]+\) )+)").unwrap();
        let captures = re.captures(line).unwrap();

        let target_lights: Vec<_> = captures["lights"].chars().map(|ch| ch == '#').collect();
        let buttons: Vec<_> = captures["buttons"].split(" ").filter(|&s| !s.is_empty()).map(Button::parse).collect();

        Self { target_lights, buttons }
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


}