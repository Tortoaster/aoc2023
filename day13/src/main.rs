use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input13");

    println!("Part One: {}", solve_13a(input));
    println!("Part Two: {}", solve_13b(input));
}

fn solve_13a(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|pattern| Pattern::from_str(pattern).unwrap())
        .map(|pattern| pattern.summarize())
        .sum()
}

fn solve_13b(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|pattern| Pattern::from_str(pattern).unwrap())
        .map(|pattern| pattern.summarize_smudged())
        .sum()
}

struct Pattern {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn summarize(&self) -> u32 {
        match self.find_horizontal_mirror() {
            None => self.find_vertical_mirror().expect("no solution") as u32,
            Some(value) => value as u32 * 100,
        }
    }

    fn find_horizontal_mirror(&self) -> Option<usize> {
        for row in 0..self.height - 1 {
            let next_row = row + 1;

            if (0..=row.min(self.height - next_row - 1))
                .all(|offset| self.tiles[row - offset] == self.tiles[next_row + offset])
            {
                return Some(next_row);
            }
        }

        None
    }

    fn find_vertical_mirror(&self) -> Option<usize> {
        for col in 0..self.width - 1 {
            let next_col = col + 1;

            if (0..=col.min(self.width - next_col - 1)).all(|offset| {
                self.tiles
                    .iter()
                    .all(|row| row[col - offset] == row[next_col + offset])
            }) {
                return Some(next_col);
            }
        }

        None
    }

    fn summarize_smudged(&self) -> u32 {
        match self.find_horizontal_mirror_smudged() {
            None => self.find_vertical_mirror_smudged().expect("no solution") as u32,
            Some(value) => value as u32 * 100,
        }
    }

    fn find_horizontal_mirror_smudged(&self) -> Option<usize> {
        let mut smudge_cleaned = false;

        for row in 0..self.height - 1 {
            let next_row = row + 1;

            if (0..=row.min(self.height - next_row - 1)).all(|offset| {
                self.tiles[row - offset]
                    .iter()
                    .zip(&self.tiles[next_row + offset])
                    .all(|(top, bottom)| {
                        *top == *bottom
                            // Ignore the first pair of tiles that is not equal
                            || (!smudge_cleaned).then(|| smudge_cleaned = true).is_some()
                    })
            }) {
                if smudge_cleaned {
                    return Some(next_row);
                }
            }
            smudge_cleaned = false;
        }

        None
    }

    fn find_vertical_mirror_smudged(&self) -> Option<usize> {
        let mut smudge_cleaned = false;

        for col in 0..self.width - 1 {
            let next_col = col + 1;

            if (0..=col.min(self.width - next_col - 1)).all(|offset| {
                self.tiles.iter().all(|row| {
                    row[col - offset] == row[next_col + offset]
                        // Ignore the first pair of tiles that is not equal
                        || (!smudge_cleaned).then(|| smudge_cleaned = true).is_some()
                })
            }) {
                if smudge_cleaned {
                    return Some(next_col);
                }
            }
            smudge_cleaned = false;
        }

        None
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        let width = tiles.first().unwrap().len();
        let height = tiles.len();

        let pattern = Pattern {
            tiles,
            width,
            height,
        };

        Ok(pattern)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Tile {
    Rock,
    Ash,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Rock,
            '.' => Self::Ash,
            _ => panic!("invalid tile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_13a, solve_13b};

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_13a() {
        assert_eq!(solve_13a(INPUT), 405)
    }

    #[test]
    fn test_13b() {
        assert_eq!(solve_13b(INPUT), 400)
    }
}
