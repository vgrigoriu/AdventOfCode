use std::str::FromStr;

use color_eyre::{
    Report, Result,
    eyre::{Context, OptionExt},
};

const INPUT: &str = include_str!("../../input/2025/day12.in");

pub fn solve1() -> Result<usize> {
    let lines = INPUT.lines().collect::<Vec<_>>();

    let regions = lines
        .split(|&l| l.is_empty())
        .next_back()
        .ok_or_eyre("invalid input")?;

    let regions: Vec<_> = regions
        .iter()
        .map(|l| l.parse::<Region>())
        .collect::<Result<_>>()?;

    let shape_tiles = vec![7, 7, 5, 6, 7, 7];
    let fitting_regions: Vec<_> = regions.iter().filter(|&r| r.fits(&shape_tiles)).collect();

    Ok(fitting_regions.len())
}

pub fn solve2() -> Result<usize> {
    todo!("a bit later")
}

#[derive(Debug)]
struct Region {
    width: u16,
    length: u16,
    shape_counts: Vec<u16>,
}

impl Region {
    fn fits(&self, shape_tiles: &[u16]) -> bool {
        self.shape_counts
            .iter()
            .zip(shape_tiles)
            .map(|(&a, &b)| a * b)
            .sum::<u16>()
            < self.width * self.length
    }
}

impl FromStr for Region {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        //50x39: 42 56 53 50 49 51
        let (dims, shape_counts) = s.split_once(": ").ok_or_eyre("expected colon")?;
        let (width, length) = dims.split_once("x").ok_or_eyre("expected x")?;
        let shape_counts: Vec<_> = shape_counts
            .split(" ")
            .map(|c| c.parse().wrap_err("invalid shape count"))
            .collect::<Result<_>>()?;

        Ok(Self {
            width: width.parse()?,
            length: length.parse()?,
            shape_counts,
        })
    }
}
