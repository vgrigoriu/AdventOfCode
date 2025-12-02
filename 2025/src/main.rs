use aoc2025::dial::Dial;

const INPUT01: &str = include_str!("../input/day01.in");
const INPUT02: &str = include_str!("../input/day02.in");

fn main() {
    let mut dial = Dial::new();
    for rotation in INPUT01.lines() {
        dial.apply(rotation);
    }

    println!("{}", dial.password());
    println!("{}", dial.real_password());

    let ranges = INPUT02.split(",");
    let mut sum = 0;
    let mut real_sum = 0;
    for range in ranges {
        let range_ends: Vec<&str> = range.split("-").collect();
        let range_start: u64 = range_ends[0].parse().unwrap();
        let range_end: u64 = range_ends[1].parse().unwrap();

        for id in range_start..=range_end {
            if is_invalid_id(id) {
                sum += id;
            }
            if is_real_invalid_id(id) {
                real_sum += id;
            }
        }
    }
    println!("{}", sum);
    println!("{}", real_sum);
}

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    if len % 2 == 1 {
        return false;
    }

    let first_half: String = id_str.chars().take(len/2).collect();
    let second_half: String = id_str.chars().skip(len/2).collect();
    return first_half == second_half;
}

fn is_real_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    for n in 1..=len/2 {
        if len % n != 0 {
            continue;
        }
        let sequence: String = id_str.chars().take(n).collect();
        if sequence.repeat(len / n) == id_str {
            return true;
        }
    }

    false
}