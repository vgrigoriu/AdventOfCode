use aoc2025::dial::Dial;

mod day05;

const INPUT01: &str = include_str!("../input/day01.in");

fn main() {
    let mut dial = Dial::new();
    for rotation in INPUT01.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.password());
    println!("{}", dial.real_password());

    day05::solve1();
    day05::solve2();
}
