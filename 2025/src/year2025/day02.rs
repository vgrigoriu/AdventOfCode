const INPUT: &str = include_str!("../../input/day02.in");

pub fn solve1() {
    println!("{}", solve(is_invalid_id));
}

pub fn solve2() {
    println!("{}", solve(is_real_invalid_id));
}

fn solve(is_invalid: fn(u64) -> bool) -> u64 {
    let ranges = INPUT.split(",");
    let mut sum = 0;
    for range in ranges {
        let range_ends: Vec<&str> = range.split("-").collect();
        let range_start: u64 = range_ends[0].parse().unwrap();
        let range_end: u64 = range_ends[1].parse().unwrap();

        for id in range_start..=range_end {
            if is_invalid(id) {
                sum += id;
            }
        }
    }
    sum
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