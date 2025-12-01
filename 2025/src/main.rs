use aoc2025::dial::Dial;

const INPUT: &str = include_str!("../input/day01.in");

fn main() {
    let mut dial = Dial::new();
    for rotation in INPUT.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.password());
    println!("{}", dial.real_password());
}
