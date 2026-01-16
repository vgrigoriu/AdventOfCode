use anyhow::Result;

use crate::utilities::parse_grid;

const INPUT: &str = include_str!("../../input/2020/day03.in");

pub fn solve1() -> Result<usize> {
    let grid = parse_grid(INPUT);

    let trees_count = count_trees(&grid, 3, 1);

    Ok(trees_count)
}

pub fn solve2() -> Result<usize> {
    let grid = parse_grid(INPUT);
    let mut result = 1;
    for (right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        result *= count_trees(&grid, right, down);
    }

    Ok(result)
}

fn count_trees(grid: &[Vec<char>], right: usize, down: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut trees_count = 0;
    let mut row = 0;
    while row < rows {
        let col = (row / down) * right % cols;
        if grid[row][col] == '#' {
            trees_count += 1;
        }
        row += down;
    }

    trees_count
}
