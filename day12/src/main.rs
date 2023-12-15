use std::str::FromStr;

use rustc_hash::FxHashMap;

type Cache<'a, 'b> = FxHashMap<(&'a [Spring], &'b [u32]), Result<u32, ()>>;

fn main() {
    let input = include_str!("../../inputs/input12");

    println!("Part One: {}", solve_12a(input));
    println!("Part Two: {}", solve_12b(input));
}

fn solve_12a(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect();

    let mut cache = Cache::default();

    lines
        .iter()
        .map(|line| count_possibilities(&line.springs, &line.segments, &mut cache))
        .sum()
}

fn solve_12b(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .map(|line| line.multiplied(5))
        .collect();

    let mut cache = Cache::default();

    lines
        .iter()
        .map(|line| {
            dbg!(count_possibilities(
                &line.springs,
                &line.segments,
                &mut cache
            ))
        })
        .sum()
}

fn count_possibilities<'a, 'b>(
    springs: &'a [Spring],
    segments: &'b [u32],
    cache: &mut Cache<'a, 'b>,
) -> u32 {
    // Get rid of trivial cases
    let springs = trim_known_operational(springs);
    if segments.iter().sum::<u32>() + segments.len() as u32 > springs.len() as u32 + 1 {
        return 0;
    }

    // Find a functional spring in the middle
    let functional = springs
        .iter()
        .enumerate()
        .find_map(|(index, spring)| spring.is_operational().then_some(index));

    match functional {
        // No known functional springs, solve easier problem
        None => count_possibilities_simple(springs, segments, cache).unwrap_or_default(),
        // At least one functional spring, split the problem and combine the results
        Some(index) => (0..=segments.len())
            .map(|segments_split_index| {
                let (left_segments, right_segments) = segments.split_at(segments_split_index);
                count_possibilities(&springs[..index], left_segments, cache)
                    * count_possibilities(&springs[index + 1..], right_segments, cache)
            })
            .sum(),
    }
}

/// Counts the possibilities for a problem with no known functional springs.
fn count_possibilities_simple<'a, 'b>(
    springs: &'a [Spring],
    segments: &'b [u32],
    cache: &mut Cache<'a, 'b>,
) -> Result<u32, ()> {
    if let Some(&result) = cache.get(&(springs, segments)) {
        return result;
    }

    match segments.first() {
        // With 0 segments, there is exactly 1 possibility
        None => Ok(1),
        // Try placing the first segment everywhere, and solve the rest of the segments recursively
        Some(&segment) => {
            if segments.iter().sum::<u32>() + segments.len() as u32 > springs.len() as u32 + 1 {
                // There is not enough space for the segments
                cache.insert((springs, segments), Err(()));
                return Err(());
            }

            if segment == springs.len() as u32 && segments.len() == 1 {
                // Last one fits snugly
                cache.insert((springs, segments), Ok(1));
                return Ok(1);
            }

            let mut total = 0;

            // Try placing the segment at the current leftmost possible position
            if !springs[segment as usize].is_broken() {
                total += count_possibilities_simple(
                    &springs[segment as usize + 1..],
                    &segments[1..],
                    cache,
                )
                .unwrap_or_default();
            }

            // Don't try placing the segment at the current leftmost possible position
            // Continue to the next unknown spring
            total += match (1..springs.len()).find(|&index| springs[index - 1].is_unknown()) {
                None => 0,
                Some(index) => match strip_broken_prefix(&springs[index..], segments) {
                    Ok((trimmed_springs, trimmed_segments)) => {
                        count_possibilities_simple(trimmed_springs, trimmed_segments, cache)
                            .unwrap_or_default()
                    }
                    Err(_) => 0,
                },
            };

            cache.insert((springs, segments), Ok(total));
            Ok(total)
        }
    }
}

#[must_use]
fn trim_known_operational(springs: &[Spring]) -> &[Spring] {
    let starts_getting_interesting = springs
        .iter()
        .copied()
        .take_while(Spring::is_operational)
        .count();
    let ends_being_interesting = springs.len()
        - springs
            .iter()
            .rev()
            .copied()
            .take_while(Spring::is_operational)
            .count();
    &springs[starts_getting_interesting..ends_being_interesting]
}

fn strip_broken_prefix<'a, 'b>(
    mut springs: &'a [Spring],
    mut segments: &'b [u32],
) -> Result<(&'a [Spring], &'b [u32]), ()> {
    assert!(!springs.contains(&Spring::Operational));

    let new_start_index = after_broken_prefix_segment_index(springs, segments)?;
    if new_start_index > 0 {
        springs = &springs[new_start_index..];
        segments = &segments[1..];
    }

    Ok((springs, segments))
}

fn after_broken_prefix_segment_index(springs: &[Spring], segments: &[u32]) -> Result<usize, ()> {
    let prefixed_known_broken = springs
        .iter()
        .copied()
        .take_while(Spring::is_broken)
        .count();

    if prefixed_known_broken > 0 {
        let Some(remaining_segment) = segments
            .first()
            .and_then(|segment| segment.checked_sub(prefixed_known_broken as u32))
        else {
            // Too many are broken for the first segment
            return Err(());
        };

        let segment_end = prefixed_known_broken + remaining_segment as usize;
        if segment_end > springs.len() {
            // Not enough space to complete the segment
            return Err(());
        }

        if springs[prefixed_known_broken..segment_end]
            .iter()
            .all(Spring::might_be_broken)
        {
            if segment_end == springs.len() {
                Ok(segment_end)
            } else if springs[segment_end].might_be_operational() {
                Ok(segment_end + 1)
            } else {
                // No space between this broken segment and the next
                Err(())
            }
        } else {
            // Too few are broken for the first segment
            Err(())
        }
    } else {
        Ok(0)
    }
}

struct Line {
    springs: Vec<Spring>,
    segments: Vec<u32>,
}

impl Line {
    fn multiplied(mut self, multiplier: u32) -> Self {
        for _ in 0..multiplier {
            self.springs.push(Spring::Unknown);
            self.springs.extend(self.springs.clone());
            self.segments.extend(self.segments.clone());
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
    fn is_operational(&self) -> bool {
        *self == Self::Operational
    }

    fn is_broken(&self) -> bool {
        *self == Self::Broken
    }

    fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

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
        assert_eq!(solve_12b(INPUT), 21)
    }
}
