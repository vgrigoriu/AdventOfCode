use std::collections::VecDeque;

const INPUT: &str = include_str!("../input/day09.in");

pub fn solve1() {
    let tiles: Vec<(u64, u64)> = INPUT.lines().map(|line|{
        let parts: Vec<_> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    })
    .collect();

    let mut max_size = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let tile1 = tiles[i];
            let tile2 = tiles[j];
            let current_size = (tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1);
            if max_size < current_size {
                max_size = current_size;
            }
        }
    }

    println!("{max_size}");
}

pub fn solve2() {
    let tiles: Vec<(usize, usize)> = INPUT.lines().map(|line|{
        let parts: Vec<_> = line.split(",").collect();
        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
    })
    .collect();

    let min_x = tiles.iter().map(|t|t.0).min().unwrap();
    let max_x = tiles.iter().map(|t|t.0).max().unwrap();
    let min_y = tiles.iter().map(|t|t.1).min().unwrap();
    let max_y = tiles.iter().map(|t|t.1).max().unwrap();

    println!("Grid will be {}lx{}c", max_y -min_y+1, max_x-min_x+1);
    println!("translaring tiles...");
    let translated_tiles: Vec<_> = tiles.iter().map(|t| (t.0 - min_x, t.1 - min_y)).collect();
    println!("allocating grid...");
    let mut grid: Vec<_> = (0..=(max_y - min_y)).map(|_| vec!['.'; max_x-min_x+1]).collect();

    // draw lines
    println!("drawing lines...");
    let mut previous_tile = translated_tiles[0];
    grid[previous_tile.1][previous_tile.0] = '#';
    for &tile in &translated_tiles[1..] {
        grid[tile.1][tile.0] = '#';
        fill_line(&mut grid, tile, previous_tile);
        previous_tile = tile;
    }
    fill_line(&mut grid, previous_tile, translated_tiles[0]);

    // fill in the shape
    // 1. find one point inside
    println!("finding a point inside...");
    let mut point_inside = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                // hit a corner, don't know how to proceed
                break;
            }
            if grid[y][x] == 'X' {
                // next one is inside
                point_inside = (x+1, y);
                break;
            }
        }
        if point_inside != (0, 0) {
            break;
        }
    }

    println!("filling in...");
    let mut queue = VecDeque::new();
    queue.push_back(point_inside);
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        if grid[point.1][point.0] != '.' {
            //already visited
            continue;
        }
        grid[point.1][point.0] = 'X';
        queue.push_back((point.0-1, point.1));
        queue.push_back((point.0+1, point.1));
        queue.push_back((point.0, point.1-1));
        queue.push_back((point.0, point.1+1));
    }

    // for line in grid {
    //     for ch in line {
    //         print!("{}", ch);
    //     }
    //     println!();
    // }
}

fn fill_line(grid: &mut [Vec<char>], tile: (usize, usize), previous_tile: (usize, usize)) {
    if previous_tile.0 == tile.0 {
        let x = tile.0;
        if previous_tile.1 < tile.1 {
            for y in (previous_tile.1+1)..tile.1 {
                grid[y][x] = 'X';
            }
        } else {
            for y in (tile.1+1)..previous_tile.1 {
                grid[y][x] = 'X';
            }
        }
    } else {
        let y = tile.1;
        if previous_tile.0 < tile.0 {
            for x in (previous_tile.0+1)..tile.0 {
                grid[y][x] = 'X';
            }
        } else {
            for x in (tile.0+1)..previous_tile.0 {
                grid[y][x] = 'X';
            }
        }
    }
}