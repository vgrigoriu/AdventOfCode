use crate::utilities::parse_grid;

const INPUT: &str = include_str!("../../input/2025/day06.in");

pub fn solve1() {
    let lines: Vec<&str> = INPUT.lines().collect();
    let number_lines: Vec<Vec<u64>> = lines[..4]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let signs = lines[4].split_whitespace();

    let mut total_sum = 0;
    for (i, sign) in signs.enumerate() {
        let op = match sign {
            "*" => u64::wrapping_mul,
            "+" => u64::wrapping_add,
            other => panic!("Unexpected operator: {other}"),
        };

        total_sum += number_lines[1..]
            .iter()
            .fold(number_lines[0][i], |acc, line| op(acc, line[i]));
    }

    println!("{}", total_sum);
}

pub fn solve2() {
    let original_lines = parse_grid(INPUT);
    let transposed = transpose(&original_lines);

    let chunks = transposed.split(|line| line.iter().all(|ch| ch.is_ascii_whitespace()));
    let result: u64 = chunks.map(solve_chunk).sum();
    println!("{}", result);
}

fn transpose(src: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = src.len();
    let cols = src[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| src[row][col]).collect())
        .collect()
}

fn solve_chunk(chunk: &[Vec<char>]) -> u64 {
    let (op, init): (fn(u64, u64) -> u64, u64) = match chunk[0].last().unwrap() {
        '*' => (u64::wrapping_mul, 1),
        '+' => (u64::wrapping_add, 0),
        other => panic!("Unexpected operator: {other}"),
    };

    chunk.iter().map(|line| parse_line(line)).fold(init, op)
}

fn parse_line(line: &[char]) -> u64 {
    line.iter()
        .filter_map(|ch| ch.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit as u64)
}
