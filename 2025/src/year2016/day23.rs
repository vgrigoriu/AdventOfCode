use color_eyre::Result;

use crate::assembunny::Computer;

const INPUT: &str = include_str!("../../input/2016/day23.in");

pub fn solve1() -> Result<i64> {
    let mut computer = Computer::new(INPUT, 7, 0, 0, 0);

    while !computer.is_done() {
        computer.step();
    }

    Ok(computer.a())
}

pub fn solve2() -> Result<i64> {
    let mut computer = Computer::new(INPUT, 12, 0, 1, 0);

    while !computer.is_done() {
        computer.step();
    }

    Ok(computer.a())
}
