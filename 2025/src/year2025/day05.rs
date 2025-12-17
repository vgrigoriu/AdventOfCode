use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

const INPUT: &str = include_str!("../../input/2025/day05.in");

pub fn solve1() {
    let ranges: Vec<RangeInclusive<u64>> = INPUT
        .lines()
        .take_while(|line| line.contains('-'))
        .map(parse_range)
        .collect();
    let ingredients = INPUT
        .lines()
        .skip_while(|line| line.contains('-') || line.is_empty())
        .map(|s| s.parse().unwrap());

    let how_many_fresh_ingredients = ingredients.filter(|i| is_fresh(*i, &ranges)).count();
    println!("{}", how_many_fresh_ingredients);
}

pub fn solve2() {
    let mut ranges: Vec<RangeInclusive<u64>> = INPUT
        .lines()
        .take_while(|line| line.contains('-'))
        .map(parse_range)
        .collect();

    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut i = 0;
    loop {
        if i == ranges.len() - 1 {
            break;
        }
        let r1 = &ranges[i];
        let r2 = &ranges[i + 1];
        if r2.start() <= r1.end() {
            let combined = combine(r1, r2);
            ranges.remove(i);
            ranges.remove(i);
            ranges.insert(i, combined);
            // Don't increment i, so we can compare with the next range.
            // We decreased ranges.len(), so we're making progress.
        } else {
            i += 1
        }
    }

    let total_fresh_ingredients: u64 = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("{}", total_fresh_ingredients);
}

fn parse_range(input_range: &str) -> RangeInclusive<u64> {
    let parts: Vec<u64> = input_range
        .split("-")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    parts[0]..=parts[1]
}

fn is_fresh(ingredient: u64, fresh_ranges: &Vec<RangeInclusive<u64>>) -> bool {
    for fresh_range in fresh_ranges {
        if fresh_range.contains(&ingredient) {
            return true;
        }
    }

    false
}

fn combine(r1: &RangeInclusive<u64>, r2: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    *min(r1.start(), r2.start())..=*max(r1.end(), r2.end())
}
