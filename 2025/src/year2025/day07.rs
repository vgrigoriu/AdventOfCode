use crate::utilities::{map_grid, parse_grid};

const INPUT: &str = include_str!("../../input/2025/day07.in");

pub fn solve1() {
    let mut manifold = parse_grid(INPUT);

    let entry_point = manifold[0].iter().position(|&ch| ch == 'S').unwrap();
    manifold[1][entry_point] = '|';

    let mut how_many_splits = 0;
    for line_no in 0..manifold.len() - 1 {
        for col_no in 0..manifold[line_no].len() {
            let ch = manifold[line_no][col_no];
            if ch == '|' {
                if manifold[line_no + 1][col_no] == '^' {
                    how_many_splits += 1;
                    manifold[line_no + 1][col_no - 1] = '|';
                    manifold[line_no + 1][col_no + 1] = '|';
                } else {
                    manifold[line_no + 1][col_no] = '|';
                }
            }
        }
    }

    println!("{how_many_splits}");
}

pub fn solve2() {
    let mut manifold = parse_grid(INPUT);
    let mut worlds = map_grid(&manifold, |_| 0u64);

    let entry_point = manifold[0].iter().position(|&ch| ch == 'S').unwrap();
    manifold[1][entry_point] = '|';
    worlds[1][entry_point] = 1;

    for line_no in 0..manifold.len() - 1 {
        for col_no in 0..manifold[line_no].len() {
            let ch = manifold[line_no][col_no];
            let current_worlds = worlds[line_no][col_no];
            if ch == '|' {
                if manifold[line_no + 1][col_no] == '^' {
                    manifold[line_no + 1][col_no - 1] = '|';
                    worlds[line_no + 1][col_no - 1] += current_worlds;
                    manifold[line_no + 1][col_no + 1] = '|';
                    worlds[line_no + 1][col_no + 1] += current_worlds;
                } else {
                    manifold[line_no + 1][col_no] = '|';
                    worlds[line_no + 1][col_no] += current_worlds;
                }
            }
        }
    }

    let how_many_worlds = worlds.last().unwrap().iter().sum::<u64>();
    println!("{}", how_many_worlds);
}
