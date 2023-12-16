use std::{collections::BTreeSet, str::FromStr};

use rayon::prelude::*;

fn main() {
    let input = include_str!("../../inputs/input16");

    println!("Part One: {}", solve_16a(input));
    println!("Part Two: {}", solve_16b(input));
}

fn solve_16a(input: &str) -> u32 {
    let mut input = input.parse::<Contraption>().unwrap();
    input.beams[0][0].insert(Direction::Right);
    input.simulate();
    input.energized()
}

fn solve_16b(input: &str) -> u32 {
    let input = input.parse::<Contraption>().unwrap();

    (0..input.height)
        .into_par_iter()
        .flat_map(|row| {
            vec![
                ((row, 0), Direction::Right),
                ((row, input.width - 1), Direction::Left),
            ]
        })
        .chain((0..input.width).into_par_iter().flat_map(|col| {
            vec![
                ((0, col), Direction::Down),
                ((input.height - 1, col), Direction::Up),
            ]
        }))
        .map(|((row, col), dir)| {
            let mut input = input.clone();
            input.beams[row][col].insert(dir);
            input.simulate();
            input.energized()
        })
        .max()
        .unwrap()
}

#[derive(Clone)]
struct Contraption {
    tiles: Vec<Vec<Tile>>,
    beams: Vec<Vec<BTreeSet<Direction>>>,
    width: usize,
    height: usize,
}

impl Contraption {
    fn energized(&self) -> u32 {
        self.beams
            .iter()
            .flatten()
            .filter(|set| !set.is_empty())
            .count() as u32
    }

    fn simulate(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        let mut changed = false;
        let old = self.beams.clone();

        for ((row, col), beams) in old.into_iter().enumerate().flat_map(|(row, beams)| {
            beams
                .into_iter()
                .enumerate()
                .map(move |(col, beam)| ((row, col), beam))
        }) {
            for &beam in &beams {
                match self.tiles[row][col] {
                    Tile::Space => {
                        if let Some(pos) = beam.next_pos((row, col), self.width, self.height) {
                            changed |= self.beams[pos.0][pos.1].insert(beam);
                        }
                    }
                    Tile::UpMirror => {
                        let beam = beam.mirror_up();
                        if let Some(pos) = beam.next_pos((row, col), self.width, self.height) {
                            changed |= self.beams[pos.0][pos.1].insert(beam);
                        }
                    }
                    Tile::DownMirror => {
                        let beam = beam.mirror_down();
                        if let Some(pos) = beam.next_pos((row, col), self.width, self.height) {
                            changed |= self.beams[pos.0][pos.1].insert(beam);
                        }
                    }
                    Tile::HorizontalSplitter => {
                        if beam.is_horizontal() {
                            if let Some(pos) = beam.next_pos((row, col), self.width, self.height) {
                                changed |= self.beams[pos.0][pos.1].insert(beam);
                            }
                        } else {
                            if let Some(col) = col.checked_sub(1) {
                                changed |= self.beams[row][col].insert(Direction::Left);
                            }
                            if let Some(col) = (col + 1 < self.width).then_some(col + 1) {
                                changed |= self.beams[row][col].insert(Direction::Right);
                            }
                        }
                    }
                    Tile::VerticalSplitter => {
                        if beam.is_vertical() {
                            if let Some(pos) = beam.next_pos((row, col), self.width, self.height) {
                                changed |= self.beams[pos.0][pos.1].insert(beam);
                            }
                        } else {
                            if let Some(row) = row.checked_sub(1) {
                                changed |= self.beams[row][col].insert(Direction::Up);
                            }
                            if let Some(row) = (row + 1 < self.height).then_some(row + 1) {
                                changed |= self.beams[row][col].insert(Direction::Down);
                            }
                        }
                    }
                }
            }
        }

        changed
    }
}

impl FromStr for Contraption {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();

        let width = tiles[0].len();
        let height = tiles.len();

        let beams = vec![vec![BTreeSet::new(); width]; height];

        Ok(Self {
            tiles,
            beams,
            width,
            height,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next_pos(&self, pos: (usize, usize), width: usize, height: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => Some((pos.0.checked_sub(1)?, pos.1)),
            Direction::Right => (pos.1 + 1 < width).then_some((pos.0, pos.1 + 1)),
            Direction::Down => (pos.0 + 1 < height).then_some((pos.0 + 1, pos.1)),
            Direction::Left => Some((pos.0, pos.1.checked_sub(1)?)),
        }
    }

    fn mirror_up(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn mirror_down(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }

    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }
}

#[derive(Copy, Clone)]
enum Tile {
    Space,
    UpMirror,
    DownMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '/' => Self::UpMirror,
            '\\' => Self::DownMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!("invalid tile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_16a, solve_16b};

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_16a() {
        assert_eq!(solve_16a(INPUT), 46)
    }

    #[test]
    fn test_16b() {
        assert_eq!(solve_16b(INPUT), 51)
    }
}
