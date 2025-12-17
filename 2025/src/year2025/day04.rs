use std::cmp::min;

use crate::utilities::parse_grid;

const INPUT: &str = include_str!("../../input/2025/day04.in");

pub fn solve1() {
    let grid = parse_grid(INPUT);

    let mut how_many_rolls = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' && count_neighbors(&grid, i, j) < 4 {
                how_many_rolls += 1;
            }
        }
    }

    println!("{}", how_many_rolls);
}

pub fn solve2() {
    let mut grid = parse_grid(INPUT);
    let mut grid_copy = grid.clone();

    let mut total_rolls_removed = 0;

    loop {
        let mut rolls_removed = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '@' && count_neighbors(&grid, i, j) < 4 {
                    rolls_removed += 1;
                    grid_copy[i][j] = '.';
                }
                // else keep old value
            }
        }
        grid = grid_copy;
        grid_copy = grid.clone();
        total_rolls_removed += rolls_removed;
        if rolls_removed == 0 {
            break;
        }
    }

    println!("{}", total_rolls_removed);
}

fn count_neighbors(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut count = 0;
    let i_min = if i == 0 { 0 } else { i - 1 };
    let i_max = min(i + 1, grid.len() - 1);
    let j_min = if j == 0 { 0 } else { j - 1 };
    let j_max = min(j + 1, grid[i].len() - 1);

    for (ii, _) in grid.iter().enumerate().take(i_max + 1).skip(i_min) {
        for (jj, _) in grid.iter().enumerate().take(j_max + 1).skip(j_min) {
            if ii == i && jj == j {
                continue;
            }
            if grid[ii][jj] == '@' {
                count += 1;
            }
        }
    }

    count
}
