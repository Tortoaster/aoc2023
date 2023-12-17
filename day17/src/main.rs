mod min_heap;

use std::str::FromStr;

use crate::min_heap::MinHeap;

fn main() {
    let input = include_str!("../../inputs/input17");

    println!("Part One: {}", solve_17a(input));
    println!("Part Two: {}", solve_17b(input));
}

fn solve_17a(input: &str) -> u32 {
    let map = Map::from_str(input).unwrap();
    map.shortest_path(Pos::new(0, 0), Pos::new(map.height - 1, map.width - 1))
}

fn solve_17b(input: &str) -> u32 {
    let map = Map::from_str(input).unwrap();
    map.shortest_ultra_path(Pos::new(0, 0), Pos::new(map.height - 1, map.width - 1))
}

struct Map {
    map: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Map {
    fn shortest_path(&self, start: Pos, end: Pos) -> u32 {
        let mut open = MinHeap::new();
        let mut closed = MinHeap::new();

        open.insert((start, Dir::East, 0), 0);

        loop {
            // Get most promising node
            let ((current_pos, last_dir, straight), cost) = open.remove_min().unwrap();
            closed.insert((current_pos, last_dir, straight), cost);

            // Base case, end reached
            if current_pos == end {
                return cost;
            }

            // Update frontier
            let mut new = Vec::new();
            let left_dir = last_dir.left();
            if let Some(pos) = current_pos.step(left_dir, self.width, self.height) {
                let key = (pos, left_dir, 1);
                let value = cost + self.map[pos.row][pos.col];
                new.push((key, value));
            }
            if straight < 3 {
                if let Some(pos) = current_pos.step(last_dir, self.width, self.height) {
                    let key = (pos, last_dir, straight + 1);
                    let value = cost + self.map[pos.row][pos.col];
                    new.push((key, value));
                }
            }
            let right_dir = last_dir.right();
            if let Some(pos) = current_pos.step(right_dir, self.width, self.height) {
                let key = (pos, right_dir, 1);
                let value = cost + self.map[pos.row][pos.col];
                new.push((key, value));
            }

            for (key, value) in new {
                if !closed.contains_key(&key) && !open.contains_key(&key) {
                    open.insert(key, value);
                }
            }
        }
    }

    fn shortest_ultra_path(&self, start: Pos, end: Pos) -> u32 {
        let mut open = MinHeap::new();
        let mut closed = MinHeap::new();

        open.insert((start, Dir::East, 0), 0);
        open.insert((start, Dir::South, 0), 0);

        loop {
            // Get most promising node
            let ((current_pos, last_dir, straight), cost) = open.remove_min().unwrap();
            closed.insert((current_pos, last_dir, straight), cost);

            // Base case, end reached
            if straight >= 4 && current_pos == end {
                return cost;
            }

            // Update frontier
            let mut new = Vec::new();
            if straight >= 4 {
                let left_dir = last_dir.left();
                if let Some(pos) = current_pos.step(left_dir, self.width, self.height) {
                    let key = (pos, left_dir, 1);
                    let value = cost + self.map[pos.row][pos.col];
                    new.push((key, value));
                }
            }
            if straight < 10 {
                if let Some(pos) = current_pos.step(last_dir, self.width, self.height) {
                    let key = (pos, last_dir, straight + 1);
                    let value = cost + self.map[pos.row][pos.col];
                    new.push((key, value));
                }
            }
            if straight >= 4 {
                let right_dir = last_dir.right();
                if let Some(pos) = current_pos.step(right_dir, self.width, self.height) {
                    let key = (pos, right_dir, 1);
                    let value = cost + self.map[pos.row][pos.col];
                    new.push((key, value));
                }
            }

            for (key, value) in new {
                if !closed.contains_key(&key) && !open.contains_key(&key) {
                    open.insert(key, value);
                }
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        _ => panic!("invalid input"),
                    })
                    .collect()
            })
            .collect();
        let width = map[0].len();
        let height = map.len();

        Ok(Self { map, width, height })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn step(&self, dir: Dir, width: usize, height: usize) -> Option<Self> {
        match dir {
            Dir::North => self.north(),
            Dir::East => self.east(width),
            Dir::South => self.south(height),
            Dir::West => self.west(),
        }
    }

    fn north(&self) -> Option<Self> {
        Some(Pos {
            row: self.row.checked_sub(1)?,
            col: self.col,
        })
    }

    fn east(&self, width: usize) -> Option<Self> {
        (self.col + 1 < width).then_some(Pos {
            row: self.row,
            col: self.col + 1,
        })
    }

    fn south(&self, height: usize) -> Option<Self> {
        (self.row + 1 < height).then_some(Pos {
            row: self.row + 1,
            col: self.col,
        })
    }

    fn west(&self) -> Option<Self> {
        Some(Pos {
            row: self.row,
            col: self.col.checked_sub(1)?,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_17a, solve_17b};

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn test_17a() {
        assert_eq!(solve_17a(INPUT), 102)
    }

    #[test]
    fn test_17b() {
        assert_eq!(solve_17b(INPUT), 94)
    }
}
