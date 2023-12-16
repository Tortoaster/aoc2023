use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/input10");

    println!("Part One: {}", solve_10a(input));
    println!("Part Two: {}", solve_10b(input));
}

fn solve_10a(input: &str) -> u32 {
    Maze::from_str(input).unwrap().solve()
}

fn solve_10b(input: &str) -> u32 {
    let mut maze = Maze::from_str(input).unwrap();
    maze.solve();
    let mut path = maze.path.clone();
    path.sort_by(|pos1, pos2| pos1.row.cmp(&pos2.row));
    let xs: BTreeMap<isize, BTreeSet<isize>> = path
        .into_iter()
        .group_by(|pos| pos.row)
        .into_iter()
        .map(|(row, cols)| (row, cols.map(|pos| pos.col).collect()))
        .collect();
    let mut total = 0;

    for row in 0..maze.height as isize {
        let mut in_ring = false;
        let xs = xs.get(&row).cloned().unwrap_or_default();
        let clean_pipes: Vec<_> = maze.pipes[row as usize]
            .iter()
            .copied()
            .enumerate()
            .map(|(index, pipe)| match xs.get(&(index as isize)) {
                None => Pipe::None,
                Some(_) => pipe,
            })
            .collect();
        let mut previous = None;
        for pipe in clean_pipes {
            match pipe {
                Pipe::UpDown => in_ring = !in_ring,
                Pipe::LeftRight => (),
                Pipe::None => {
                    if in_ring {
                        total += 1;
                    }
                }
                Pipe::Start => unreachable!(),
                pipe => match previous {
                    None => previous = Some(pipe),
                    Some(opener) => {
                        previous = None;
                        if pipe
                            .directions()
                            .unwrap()
                            .into_iter()
                            .find(Direction::is_vertical)
                            .unwrap()
                            != opener
                                .directions()
                                .unwrap()
                                .into_iter()
                                .find(Direction::is_vertical)
                                .unwrap()
                        {
                            in_ring = !in_ring;
                        }
                    }
                },
            }
        }
    }

    total
}

struct Maze {
    pipes: Vec<Vec<Pipe>>,
    height: usize,
    start: Pos,
    path: Vec<Pos>,
}

impl Maze {
    fn solve(&mut self) -> u32 {
        for assumption in Pipe::ALL {
            let mut length = 0;
            let mut current_pos = self.start;
            self.path.push(current_pos);
            let mut last_direction = assumption.directions().unwrap()[0];

            current_pos = current_pos.move_in(last_direction);
            self.path.push(current_pos);
            length += 1;

            loop {
                match self.get(current_pos) {
                    None | Some(Pipe::None) => {
                        self.path.clear();
                        break;
                    }
                    Some(Pipe::Start) => {
                        *self.get_mut(self.start).unwrap() = assumption;
                        return length / 2;
                    }
                    Some(current) => {
                        let directions: Vec<_> = current
                            .directions()
                            .unwrap()
                            .into_iter()
                            .filter(|dir| *dir != last_direction.opposite())
                            .collect();
                        if directions.len() == 1 {
                            last_direction = directions[0];
                            current_pos = current_pos.move_in(last_direction);
                            self.path.push(current_pos);
                            length += 1;
                        }
                    }
                }
            }
        }

        0
    }

    fn get(&self, pos: Pos) -> Option<Pipe> {
        if pos.row < 0 || pos.col < 0 {
            None
        } else {
            Some(self.pipes[pos.row as usize][pos.col as usize])
        }
    }

    fn get_mut(&mut self, pos: Pos) -> Option<&mut Pipe> {
        if pos.row < 0 || pos.col < 0 {
            None
        } else {
            Some(&mut self.pipes[pos.row as usize][pos.col as usize])
        }
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pipes = Vec::new();
        let mut start = None;

        for (row, line) in s.lines().enumerate() {
            let mut current_row = Vec::new();

            for (col, c) in line.chars().enumerate() {
                let pipe = Pipe::from_char(c);
                if pipe == Pipe::Start {
                    start = Some(Pos {
                        row: row as isize,
                        col: col as isize,
                    });
                }
                current_row.push(pipe);
            }

            pipes.push(current_row);
        }

        let height = pipes.len();

        Ok(Maze {
            pipes,
            height,
            start: start.unwrap(),
            path: Vec::new(),
        })
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Pipe {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Start,
    None,
}

impl Pipe {
    const ALL: [Pipe; 6] = [
        Pipe::UpDown,
        Pipe::LeftRight,
        Pipe::UpRight,
        Pipe::UpLeft,
        Pipe::DownLeft,
        Pipe::DownRight,
    ];

    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe::UpDown,
            '-' => Pipe::LeftRight,
            'L' => Pipe::UpRight,
            'J' => Pipe::UpLeft,
            '7' => Pipe::DownLeft,
            'F' => Pipe::DownRight,
            'S' => Pipe::Start,
            _ => Pipe::None,
        }
    }

    fn directions(&self) -> Option<[Direction; 2]> {
        match self {
            Pipe::UpDown => Some([Direction::Up, Direction::Down]),
            Pipe::LeftRight => Some([Direction::Left, Direction::Right]),
            Pipe::UpRight => Some([Direction::Up, Direction::Right]),
            Pipe::UpLeft => Some([Direction::Up, Direction::Left]),
            Pipe::DownLeft => Some([Direction::Down, Direction::Left]),
            Pipe::DownRight => Some([Direction::Down, Direction::Right]),
            Pipe::Start | Pipe::None => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Direction::Up | Direction::Down => true,
            Direction::Right | Direction::Left => false,
        }
    }
}

#[derive(Copy, Clone)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    pub fn move_in(self, dir: Direction) -> Pos {
        match dir {
            Direction::Up => Pos {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Right => Pos {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Down => Pos {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Pos {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_10b;

    #[test]
    fn test_10b() {
        const INPUT: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(solve_10b(INPUT), 4)
    }
}
