extern crate core;

use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/input5");

    println!("Part One: {}", solve_5a(input));
    println!("Part Two: {}", solve_5b(input));
}

fn solve_5a(input: &str) -> u32 {
    let input = Input::from_str(input).unwrap();

    input
        .seeds
        .into_iter()
        .map(|mut seed| {
            for map in &input.maps {
                seed = map.get(seed).0;
            }
            seed
        })
        .min()
        .unwrap()
}

fn solve_5b(input: &str) -> u32 {
    let input = Input::from_str(input).unwrap();

    input
        .seeds
        .into_iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .map(|(start_index, length)| {
            let mut current_seed = start_index;
            let mut smallest_seed = u32::MAX;

            while current_seed < start_index + length {
                let mut smallest_margin = u32::MAX;
                let mut mapped_seed = current_seed;
                for map in &input.maps {
                    let (new_seed, margin) = map.get(mapped_seed);
                    mapped_seed = new_seed;
                    smallest_margin = smallest_margin.min(margin);
                }
                smallest_seed = smallest_seed.min(mapped_seed);
                // The margin indicates how many subsequent seeds will pass through exactly the
                // same filters, and will therefore have consecutive (greater) values. We can
                // skip those, as they won't be the smallest.
                current_seed += smallest_margin.max(1);
            }

            smallest_seed
        })
        .min()
        .unwrap()
}

struct Input {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, maps) = s.trim().split_once("\n\n").unwrap();
        let seeds = seeds
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|n| u32::from_str(n).unwrap())
            .collect();
        let maps = maps
            .split("\n\n")
            .map(|line| Map::from_str(line).unwrap())
            .collect();

        Ok(Input { seeds, maps })
    }
}

struct Map {
    rearrangers: Vec<Rearranger>,
    source: String,
    destination: String,
}

impl Map {
    fn get(&self, mut index: u32) -> (u32, u32) {
        let mut margin = u32::MAX;
        for rearranger in &self.rearrangers {
            match rearranger.rearrange(index) {
                None => (),
                Some((new_index, new_margin)) => {
                    index = new_index;
                    margin = margin.min(new_margin);
                    break;
                }
            }
        }
        (index, margin)
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ty, s) = s.split_once(' ').unwrap();
        let (source, destination) = ty.split_once("-to-").unwrap();
        let s = s.strip_prefix("map:\n").unwrap();
        let rearrangers = s
            .split('\n')
            .map(|line| Rearranger::from_str(line).unwrap())
            .collect();

        Ok(Map {
            rearrangers,
            source: source.to_owned(),
            destination: destination.to_owned(),
        })
    }
}

struct Rearranger {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl Rearranger {
    fn rearrange(&self, index: u32) -> Option<(u32, u32)> {
        match index.checked_sub(self.source_start) {
            Some(relative_index) if relative_index < self.length => Some((
                self.destination_start + relative_index,
                self.length - relative_index - 1,
            )),
            _ => None,
        }
    }
}

impl FromStr for Rearranger {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s.split(' ').map(|n| u32::from_str(n).unwrap()).collect();
        Ok(Rearranger {
            destination_start: nums[0],
            source_start: nums[1],
            length: nums[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_5a, solve_5b};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_5a() {
        assert_eq!(solve_5a(INPUT), 35)
    }

    #[test]
    fn test_5b() {
        assert_eq!(solve_5b(INPUT), 46)
    }
}
