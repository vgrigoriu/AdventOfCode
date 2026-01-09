use std::{str::FromStr, thread};

use color_eyre::{
    Result,
    eyre::{Context, OptionExt},
};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/2023/day05.in");

pub fn solve1() -> Result<i64> {
    let parts: Vec<_> = INPUT.split("\n\n").collect();
    let seeds: Vec<i64> = parts[0]
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<i64>().wrap_err("oops"))
        .collect::<Result<Vec<_>>>()?;

    let seed_to_soil_map: Map = parts[1].trim_start_matches("seed-to-soil map:\n").parse()?;
    let soil_to_fertilizer_map: Map = parts[2]
        .trim_start_matches("soil-to-fertilizer map:\n")
        .parse()?;
    let fertilizer_to_water_map: Map = parts[3]
        .trim_start_matches("fertilizer-to-water map:\n")
        .parse()?;
    let water_to_light_map: Map = parts[4]
        .trim_start_matches("water-to-light map:\n")
        .parse()?;
    let light_to_temperature_map: Map = parts[5]
        .trim_start_matches("light-to-temperature map:\n")
        .parse()?;
    let temperature_to_humidity_map: Map = parts[6]
        .trim_start_matches("temperature-to-humidity map:\n")
        .parse()?;
    let humidity_to_location_map: Map = parts[7]
        .trim_start_matches("humidity-to-location map:\n")
        .parse()?;

    let result = seeds
        .iter()
        .map(|&seed| seed_to_soil_map.map(seed))
        .map(|soil| soil_to_fertilizer_map.map(soil))
        .map(|fertilizer| fertilizer_to_water_map.map(fertilizer))
        .map(|water| water_to_light_map.map(water))
        .map(|light| light_to_temperature_map.map(light))
        .map(|temperature| temperature_to_humidity_map.map(temperature))
        .map(|humidity| humidity_to_location_map.map(humidity))
        .min()
        .unwrap();

    Ok(result)
}

pub fn solve2() -> Result<i64> {
    let parts: Vec<_> = INPUT.split("\n\n").collect();
    let seed_ranges = parts[0]
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .tuples()
        .map(|(start, length)| start..(start+length))
        .collect::<Vec<_>>();

    let seed_to_soil_map: Map = parts[1].trim_start_matches("seed-to-soil map:\n").parse()?;
    let soil_to_fertilizer_map: Map = parts[2]
        .trim_start_matches("soil-to-fertilizer map:\n")
        .parse()?;
    let fertilizer_to_water_map: Map = parts[3]
        .trim_start_matches("fertilizer-to-water map:\n")
        .parse()?;
    let water_to_light_map: Map = parts[4]
        .trim_start_matches("water-to-light map:\n")
        .parse()?;
    let light_to_temperature_map: Map = parts[5]
        .trim_start_matches("light-to-temperature map:\n")
        .parse()?;
    let temperature_to_humidity_map: Map = parts[6]
        .trim_start_matches("temperature-to-humidity map:\n")
        .parse()?;
    let humidity_to_location_map: Map = parts[7]
        .trim_start_matches("humidity-to-location map:\n")
        .parse()?;

    let min_location = thread::scope(|s| {
        let handles: Vec<_> = seed_ranges
            .iter()
            .map(|seed_range| {
                s.spawn(|| {
                    seed_range.clone()
                        .map(|seed| seed_to_soil_map.map(seed))
                        .map(|soil| soil_to_fertilizer_map.map(soil))
                        .map(|fertilizer| fertilizer_to_water_map.map(fertilizer))
                        .map(|water| water_to_light_map.map(water))
                        .map(|light| light_to_temperature_map.map(light))
                        .map(|temperature| temperature_to_humidity_map.map(temperature))
                        .map(|humidity| humidity_to_location_map.map(humidity))
                        .min()
                        .unwrap()
                })
            })
            .collect();

        handles.into_iter()
            .map(|h| h.join().unwrap())
            .min()
            .unwrap()
    });

    Ok(min_location)
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
struct MappedRange {
    dest_start: i64,
    src_start: i64,
    length: i64,
}

impl MappedRange {
    pub fn map(&self, n: i64) -> Option<i64> {
        let src_range = self.src_start..(self.src_start + self.length);

        if src_range.contains(&n) {
            Some(n + self.dest_start - self.src_start)
        } else {
            None
        }
    }
}

impl FromStr for MappedRange {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(3, ' ');
        let dest_start = parts.next().ok_or_eyre("dest not a number")?.parse()?;
        let src_start = parts.next().ok_or_eyre("src not a number")?.parse()?;
        let length = parts.next().ok_or_eyre("length not a number")?.parse()?;

        Ok(Self {
            dest_start,
            src_start,
            length,
        })
    }
}

struct Map {
    mapped_ranges: Vec<MappedRange>,
}

impl Map {
    pub fn map(&self, n: i64) -> i64 {
        for mr in self.mapped_ranges.iter() {
            if let Some(dest) = mr.map(n) {
                return dest;
            }
        }

        n
    }
}

impl FromStr for Map {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Map {
            mapped_ranges: s
                .lines()
                .map(|l| l.parse::<MappedRange>())
                .sorted_by_key(|r| match r {
                    Ok(map) => *map,
                    Err(_) => MappedRange::default(),
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapped_range_parsing() -> Result<()> {
        let mapped_range: MappedRange = "50 98 2".parse()?;
        assert_eq!(
            mapped_range,
            MappedRange {
                dest_start: 50,
                src_start: 98,
                length: 2
            }
        );

        Ok(())
    }

    #[test]
    fn test_mapped_range_mapping() {
        let range = MappedRange {
            dest_start: 50,
            src_start: 98,
            length: 2,
        };

        assert_eq!(range.map(97), None);
        assert_eq!(range.map(98), Some(50));
        assert_eq!(range.map(99), Some(51));
        assert_eq!(range.map(100), None);
    }

    #[test]
    fn test_map_mapping() {
        let map = Map {
            mapped_ranges: vec!["50 98 2".parse().unwrap(), "52 50 48".parse().unwrap()],
        };

        assert_eq!(map.map(49), 49);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(75), 77);
        assert_eq!(map.map(97), 99);
        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(99), 51);
        assert_eq!(map.map(100), 100);
    }
}
