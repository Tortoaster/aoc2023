use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../inputs/input14");

    println!("Part One: {}", solve_14a(input));
    println!("Part Two: {}", solve_14b(input));
}

fn solve_14a(input: &str) -> u32 {
    let mut platform = Platform::from_str(input).unwrap();
    platform.tilt_up();
    platform.north_load()
}

fn solve_14b(input: &str) -> u32 {
    let mut platform = Platform::from_str(input).unwrap();
    let mut seen = HashMap::new();
    seen.insert(platform.tiles.clone(), 0);
    for cycles in 1..=1000000000 {
        platform.tilt_up();
        platform.tilt_left();
        platform.tilt_down();
        platform.tilt_right();

        if let Some(previous_cycle) = seen.get(&platform.tiles) {
            let repeating_cycle = cycles - *previous_cycle;
            let remaining_cycles = 1000000000 - cycles;
            let remaining_cycles = remaining_cycles % repeating_cycle;
            for _ in 0..remaining_cycles {
                platform.tilt_up();
                platform.tilt_left();
                platform.tilt_down();
                platform.tilt_right();
            }
            break;
        }

        seen.insert(platform.tiles.clone(), cycles);
    }
    platform.north_load()
}

struct Platform {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Platform {
    fn north_load(&self) -> u32 {
        self.tiles
            .iter()
            .rev()
            .enumerate()
            .map(|(index, row)| (1 + index as u32, row))
            .map(|(multiplier, row)| {
                row.iter().filter(|tile| **tile == Tile::Round).count() as u32 * multiplier
            })
            .sum()
    }

    fn tilt_up(&mut self) {
        for col in 0..self.width {
            let mut start_row = 0;

            while start_row < self.height {
                let mut row = start_row;
                let mut round_rocks = 0;

                while row < self.height && self.tiles[row][col] != Tile::Square {
                    if self.tiles[row][col] == Tile::Round {
                        round_rocks += 1;
                        self.tiles[row][col] = Tile::Space;
                    }

                    row += 1;
                }

                for row in start_row..start_row + round_rocks {
                    self.tiles[row][col] = Tile::Round;
                }

                start_row = row + 1;
            }
        }
    }

    fn tilt_left(&mut self) {
        for row in 0..self.height {
            let mut start_col = 0;

            while start_col < self.width {
                let mut col = start_col;
                let mut round_rocks = 0;

                while col < self.width && self.tiles[row][col] != Tile::Square {
                    if self.tiles[row][col] == Tile::Round {
                        round_rocks += 1;
                        self.tiles[row][col] = Tile::Space;
                    }

                    col += 1;
                }

                for col in start_col..start_col + round_rocks {
                    self.tiles[row][col] = Tile::Round;
                }

                start_col = col + 1;
            }
        }
    }

    fn tilt_down(&mut self) {
        for col in 0..self.width as isize {
            let mut start_row = self.height as isize - 1;

            while start_row >= 0 {
                let mut row = start_row;
                let mut round_rocks = 0;

                while row >= 0 && self.tiles[row as usize][col as usize] != Tile::Square {
                    if self.tiles[row as usize][col as usize] == Tile::Round {
                        round_rocks += 1;
                        self.tiles[row as usize][col as usize] = Tile::Space;
                    }

                    row -= 1;
                }

                for row in start_row - round_rocks + 1..=start_row {
                    self.tiles[row as usize][col as usize] = Tile::Round;
                }

                start_row = row - 1;
            }
        }
    }

    fn tilt_right(&mut self) {
        for row in 0..self.height as isize {
            let mut start_col = self.width as isize - 1;

            while start_col >= 0 {
                let mut col = start_col;
                let mut round_rocks = 0;

                while col >= 0 && self.tiles[row as usize][col as usize] != Tile::Square {
                    if self.tiles[row as usize][col as usize] == Tile::Round {
                        round_rocks += 1;
                        self.tiles[row as usize][col as usize] = Tile::Space;
                    }

                    col -= 1;
                }

                for col in start_col - round_rocks + 1..=start_col {
                    self.tiles[row as usize][col as usize] = Tile::Round;
                }

                start_col = col - 1;
            }
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Space => write!(f, ".")?,
                    Tile::Round => write!(f, "O")?,
                    Tile::Square => write!(f, "#")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();

        let width = tiles[0].len();
        let height = tiles.len();

        Ok(Platform {
            tiles,
            width,
            height,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Tile {
    Space,
    Round,
    Square,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            'O' => Self::Round,
            '#' => Self::Square,
            _ => panic!("invalid tile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_14a, solve_14b};

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_14a() {
        assert_eq!(solve_14a(INPUT), 136)
    }

    #[test]
    fn test_14b() {
        assert_eq!(solve_14b(INPUT), 64)
    }
}
