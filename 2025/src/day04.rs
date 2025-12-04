use std::cmp::{max, min};

const INPUT: &str = include_str!("../input/day04.in");

pub fn solve1() {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in INPUT.lines() {
        let grid_line: Vec<char> = line.chars().collect();
        grid.push(grid_line);
    }
    
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
    let mut grid: Vec<Vec<char>> = vec![];
    for line in INPUT.lines() {
        let grid_line: Vec<char> = line.chars().collect();
        grid.push(grid_line);
    }
    let mut grid_copy = grid.clone();

    let mut rolls_removed = 0;
    let mut total_rolls_removed = 0;

    loop {
        rolls_removed = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '@' && count_neighbors(&grid, i, j) < 4 {
                    rolls_removed += 1;
                    grid_copy[i][j] = '.';
                }
                // else keep old value
            }
        }
        // println!();
        // for i in 0..grid.len() {
        //     for j in 0..grid[i].len() {
        //         print!("{}", grid_copy[i][j]);
        //     }
        //     println!();
        // }
        grid = grid_copy;
        grid_copy = grid.clone();
        total_rolls_removed += rolls_removed;
        if rolls_removed == 0 {
            break;
        }
    }

    println!("{}", total_rolls_removed);
}

fn count_neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut count = 0;
    let i_min = if i == 0 {0} else {i - 1};
    let i_max = min (i + 1, grid.len() -1);
    let j_min = if j == 0 {0} else {j-1};
    let j_max = min(j + 1, grid[i].len() - 1);

    //println!("({}, {}): {} - {} && {} - {}", i, j, i_min, i_max, j_min, j_max);
    for ii in i_min..=i_max {
        for jj in j_min..=j_max {
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