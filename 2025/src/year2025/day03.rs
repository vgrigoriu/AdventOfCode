const INPUT: &str = include_str!("../../input/day03.in");

pub fn solve1() {
    println!("{}", solve(2));
}

pub fn solve2() {
    println!("{}", solve(12));
}

fn solve(remaining_digits: usize) -> u64 {
    let lines = INPUT.lines();
    let mut sum = 0;
    for line in lines {
        let mut digits: Vec<char> = line.chars().collect();
        while digits.len() > remaining_digits {
            let index_to_remove = find_index_of_first_digit_smaller_than_next(&digits);
            digits.remove(index_to_remove);
        }
        sum += digits_slice_to_u64(&digits);
    }

    sum
}

fn find_index_of_first_digit_smaller_than_next(digits: &[char]) -> usize {
    for (index, digit) in digits[..digits.len() - 1].iter().enumerate() {
        if *digit < digits[index + 1] {
            return index;
        }
    }

    digits.len() - 1
}

fn digits_slice_to_u64(digits: &[char]) -> u64 {
    let mut result = 0;
    for digit in digits {
        result = result * 10 + (*digit as u64 - '0' as u64)
    }
    result
}
