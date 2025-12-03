use aoc2025::dial::Dial;

mod day04;

const INPUT01: &str = include_str!("../input/day01.in");

fn main() {
    let mut dial = Dial::new();
    for rotation in INPUT01.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.password());
    println!("{}", dial.real_password());

    day04::solve1();
    day04::solve2();
}
