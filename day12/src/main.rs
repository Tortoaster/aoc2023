use std::{str::FromStr, time::Instant};

use cached::proc_macro::cached;

fn main() {
    let input = include_str!("../../inputs/input12");

    let start = Instant::now();
    println!(
        "Part One: {} ({}µs)",
        solve_12a(input),
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    println!(
        "Part Two: {} ({}µs)",
        solve_12b(input),
        start.elapsed().as_micros()
    );
}

fn solve_12a(input: &str) -> u64 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect();

    lines
        .into_iter()
        .map(|line| Box::leak(Box::new(line)))
        .map(|line| possibilities(&line.springs, &line.segments))
        .sum()
}

fn solve_12b(input: &str) -> u64 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .map(|line| line.multiplied(5))
        .collect();

    lines
        .into_iter()
        .map(|line| Box::leak(Box::new(line)))
        .map(|line| possibilities(&line.springs, &line.segments))
        .sum()
}

#[cached]
fn possibilities(springs: &'static [Spring], segments: &'static [u32]) -> u64 {
    match springs.first() {
        None => {
            if segments.is_empty() {
                1
            } else {
                0
            }
        }
        Some(Spring::Operational) => possibilities_first_operational(springs, segments),
        Some(Spring::Broken) => possibilities_first_broken(springs, segments),
        Some(Spring::Unknown) => {
            let operational = possibilities_first_operational(springs, segments);
            let broken = possibilities_first_broken(springs, segments);
            operational + broken
        }
    }
}

fn possibilities_first_broken(springs: &'static [Spring], segments: &'static [u32]) -> u64 {
    match segments.first() {
        None => 0,
        Some(&segment) => {
            if springs.len() >= segment as usize
                && springs
                    .iter()
                    .take(segment as usize)
                    .all(Spring::might_be_broken)
            {
                if springs.len() == segment as usize {
                    if segments.len() == 1 {
                        1
                    } else {
                        0
                    }
                } else if springs[segment as usize].might_be_operational() {
                    possibilities(&springs[segment as usize + 1..], &segments[1..])
                } else {
                    0
                }
            } else {
                0
            }
        }
    }
}

fn possibilities_first_operational(springs: &'static [Spring], segments: &'static [u32]) -> u64 {
    possibilities(&springs[1..], segments)
}

struct Line {
    springs: Vec<Spring>,
    segments: Vec<u32>,
}

impl Line {
    fn multiplied(mut self, multiplier: u32) -> Self {
        let original_springs = self.springs.clone();
        let original_segments = self.segments.clone();

        for _ in 0..multiplier - 1 {
            self.springs.push(Spring::Unknown);
            self.springs.extend(original_springs.clone());
            self.segments.extend(original_segments.clone());
        }

        self
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, segments) = s.split_once(' ').unwrap();
        Ok(Line {
            springs: springs.chars().map(Spring::from).collect(),
            segments: segments.split(',').map(|n| n.parse().unwrap()).collect(),
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

impl Spring {
    fn might_be_broken(&self) -> bool {
        *self != Self::Operational
    }

    fn might_be_operational(&self) -> bool {
        *self != Self::Broken
    }
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => panic!("invalid spring"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_12a, solve_12b};

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_12a() {
        assert_eq!(solve_12a(INPUT), 21)
    }

    #[test]
    fn test_12b() {
        assert_eq!(solve_12b(INPUT), 525152)
    }
}
