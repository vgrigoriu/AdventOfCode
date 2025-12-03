use aoc2025::dial::Dial;

mod day02;
mod day03;

const INPUT01: &str = include_str!("../input/day01.in");

fn main() {
    let mut dial = Dial::new();
    for rotation in INPUT01.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.password());
    println!("{}", dial.real_password());

    day02::solve1();
    day02::solve2();
    day03::solve1();
    day03::solve2();
}
