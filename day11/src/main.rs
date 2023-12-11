use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../inputs/input11");

    println!("Part One: {}", solve_11a(input));
    println!("Part Two: {}", solve_11b(input));
}

fn solve_11a(input: &str) -> u32 {
    let input: Input = input.parse().unwrap();
    let rows = input.empty_rows();
    let cols = input.empty_columns();
    input.summed_distances(&rows, &cols, 2) as u32
}

fn solve_11b(input: &str) -> u64 {
    let input: Input = input.parse().unwrap();
    let rows = input.empty_rows();
    let cols = input.empty_columns();
    input.summed_distances(&rows, &cols, 1000000)
}

struct Input {
    galaxies: Vec<(usize, usize)>,
    rows: Vec<Vec<Tile>>,
}

impl Input {
    fn summed_distances(&self, rows: &BTreeSet<usize>, cols: &BTreeSet<usize>, age: usize) -> u64 {
        let mut galaxies = self.galaxies.clone();
        let mut sum = 0;

        let mut current = galaxies.pop();
        while let Some((row, col)) = current {
            for &(row2, col2) in &galaxies {
                let smallest_row = row.min(row2);
                let biggest_row = row.max(row2);
                let smallest_col = col.min(col2);
                let biggest_col = col.max(col2);

                for _ in rows
                    .iter()
                    .copied()
                    .filter(|r| smallest_row <= *r && *r < biggest_row)
                {
                    sum += age - 1;
                }
                for _ in cols
                    .iter()
                    .copied()
                    .filter(|r| smallest_col <= *r && *r < biggest_col)
                {
                    sum += age - 1;
                }
                sum += biggest_row - smallest_row + biggest_col - smallest_col;
            }
            current = galaxies.pop();
        }

        sum as u64
    }

    fn empty_rows(&self) -> BTreeSet<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|tile| *tile == Tile::Space))
            .map(|(index, _)| index)
            .collect()
    }

    fn empty_columns(&self) -> BTreeSet<usize> {
        let mut empty_columns = BTreeSet::new();

        for column_index in 0..self.rows[0].len() {
            if self.rows.iter().all(|row| row[column_index] == Tile::Space) {
                empty_columns.insert(column_index);
            }
        }

        empty_columns
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let rows = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .map(Tile::from_char)
                    .enumerate()
                    .map(|(col, tile)| {
                        if tile == Tile::Galaxy {
                            galaxies.push((row, col))
                        }
                        tile
                    })
                    .collect()
            })
            .collect();

        Ok(Input { galaxies, rows })
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Space,
    Galaxy,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("invalid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => write!(f, "."),
            Tile::Galaxy => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_11a;

    #[test]
    fn test_11a() {
        const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(solve_11a(INPUT), 374)
    }
}
