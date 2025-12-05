use aoc2025::dial::Dial;

const INPUT: &str = include_str!("../input/day01.in");

pub fn solve1() {
    let mut dial = Dial::new();
    for rotation in INPUT.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.password());
}

pub fn solve2() {
        let mut dial = Dial::new();
    for rotation in INPUT.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.real_password());
}