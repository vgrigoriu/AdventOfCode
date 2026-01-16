use anyhow::Result;

const INPUT: &str = include_str!("../../input/2020/day02.in");

pub fn solve1() -> Result<usize> {
    let count_valid_passwords = INPUT
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(policy, password)| (policy.into(), password))
        .filter(|(policy, password): &(Policy, &str)| policy.evaluate(password))
        .count();

    Ok(count_valid_passwords)
}
pub fn solve2() -> Result<usize> {
    let count_valid_passwords = INPUT
        .lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(policy, password)| (policy.into(), password))
        .filter(|(policy, password): &(RealPolicy, &str)| policy.evaluate(password))
        .count();

    Ok(count_valid_passwords)
}

#[derive(Debug, PartialEq)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

impl Policy {
    fn evaluate(&self, password: &str) -> bool {
        (self.min..=self.max).contains(&password.chars().filter(|&c| c == self.letter).count())
    }
}

impl From<&str> for Policy {
    fn from(s: &str) -> Self {
        // 1-3 a
        let (min_max, letter) = s.split_once(' ').unwrap();
        let (min, max) = min_max.split_once('-').unwrap();

        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
            letter: letter.chars().nth(0).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct RealPolicy {
    pos1: usize,
    pos2: usize,
    letter: char,
}

impl RealPolicy {
    fn evaluate(&self, password: &str) -> bool {
        (password.chars().nth(self.pos1 - 1).unwrap() == self.letter) ^ (password.chars().nth(self.pos2 - 1).unwrap() == self.letter)
    }
}

impl From<&str> for RealPolicy {
    fn from(s: &str) -> Self {
        // 1-3 a
        let (positions, letter) = s.split_once(' ').unwrap();
        let (pos1, pos2) = positions.split_once('-').unwrap();

        Self {
            pos1: pos1.parse().unwrap(),
            pos2: pos2.parse().unwrap(),
            letter: letter.chars().nth(0).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_parsing() {
        let policy: Policy = "1-3: a".into();

        assert_eq!(
            policy,
            Policy {
                min: 1,
                max: 3,
                letter: 'a'
            }
        )
    }

    #[test]
    fn test_policy_evaluation() {
        let policy: Policy = "1-3: a".into();

        assert_eq!(policy.evaluate("abcde"), true);
        assert_eq!(policy.evaluate("cdefg"), false);
    }
}
