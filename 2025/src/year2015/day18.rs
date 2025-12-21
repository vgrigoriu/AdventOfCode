use color_eyre::Result;

use crate::utilities::{map_grid, parse_grid};

const INPUT: &str = include_str!("../../input/2015/day18.in");

pub fn solve1() -> Result<usize> {
    let mut lights = Lights::new(parse_grid(INPUT));
    for _ in 0..100 {
        lights.step();
    }
    Ok(lights.count_lights_on())
}

pub fn solve2() -> Result<usize> {
    let mut lights = Lights::new(parse_grid(INPUT));
    for _ in 0..100 {
        lights.step_2();
    }
    Ok(lights.count_lights_on())
}

struct Lights {
    lights: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Lights {
    fn new(lights: Vec<Vec<char>>) -> Self {
        let rows = lights.len();
        let cols = lights[0].len();
        Self { lights, rows, cols }
    }

    fn step(&mut self) {
        let mut next_frame = map_grid(&self.lights, |_| '*');

        for (row, r) in next_frame.iter_mut().enumerate() {
            for (col, c) in r.iter_mut().enumerate() {
                *c = match (self.is_on(row, col), self.count_neighbors(row, col)) {
                    (true, 2 | 3) => '#',
                    (true, _) => '.',
                    (false, 3) => '#',
                    (false, _) => '.',
                }
            }
        }

        std::mem::swap(&mut self.lights, &mut next_frame);
    }

    fn step_2(&mut self) {
        let mut next_frame = map_grid(&self.lights, |_| '*');

        for (row, r) in next_frame.iter_mut().enumerate() {
            for (col, c) in r.iter_mut().enumerate() {
                *c = match (
                    row,
                    col,
                    self.is_on(row, col),
                    self.count_neighbors(row, col),
                ) {
                    // Corner lights are stuck.
                    (r, c, _, _)
                        if (r == 0 || r == self.rows - 1) && (c == 0 || c == self.cols - 1) =>
                    {
                        '#'
                    }
                    (_, _, true, 2 | 3) => '#',
                    (_, _, true, _) => '.',
                    (_, _, false, 3) => '#',
                    (_, _, false, _) => '.',
                }
            }
        }

        std::mem::swap(&mut self.lights, &mut next_frame);
    }

    fn count_lights_on(&self) -> usize {
        self.lights
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&c| *c == '#')
            .count()
    }

    fn count_neighbors(&self, c_row: usize, c_col: usize) -> u8 {
        let mut neighbors = 0;

        for row in c_row.saturating_sub(1)..=c_row + 1 {
            for col in c_col.saturating_sub(1)..=c_col + 1 {
                if row == c_row && col == c_col {
                    // Skip the center light.
                    continue;
                }

                if self.within_bounds(row, col) && self.is_on(row, col) {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn within_bounds(&self, row: usize, col: usize) -> bool {
        // No need to check 0 <= row because usize is always non-negative.
        row < self.rows && col < self.cols
    }

    fn is_on(&self, row: usize, col: usize) -> bool {
        debug_assert!(self.within_bounds(row, col));

        self.lights[row][col] == '#'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_within_bounds() {
        let lights = Lights::new(parse_grid("123\n456\n789"));

        assert!(lights.within_bounds(0, 0));
        assert!(lights.within_bounds(0, 2));
        assert!(!lights.within_bounds(0, 3));
        assert!(lights.within_bounds(1, 1));
        assert!(!lights.within_bounds(3, 0));
        assert!(!lights.within_bounds(3, 7));
    }

    #[test]
    fn test_count_neighbors() {
        let lights = Lights::new(parse_grid("###\n..#\n.#."));

        assert_eq!(lights.count_neighbors(0, 0), 1);
        assert_eq!(lights.count_neighbors(1, 1), 5);
        assert_eq!(lights.count_neighbors(2, 2), 2);
    }
}
