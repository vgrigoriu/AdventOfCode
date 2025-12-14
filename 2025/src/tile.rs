use anyhow::{Context, Error, Result};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile(pub usize, pub usize);

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Tile> {
        let (part1, part2) = s.split_once(",").context("missing comma")?;
        Ok(Tile(part1.parse()?, part2.parse()?))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_tiles() {
        assert_eq!("1,2".parse::<Tile>().unwrap(), Tile(1, 2));
        assert!("cici".parse::<Tile>().is_err());
        assert!("mimi,gigi".parse::<Tile>().is_err());
    }
}
