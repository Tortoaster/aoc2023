use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input10");

    println!("Part One: {}", solve_10a(input));
    println!("Part Two: {}", solve_10b(input));
}

fn solve_10a(input: &str) -> u32 {
    Maze::from_str(input).unwrap().solve()
}

fn solve_10b(input: &str) -> u32 {
    todo!()
}

struct Maze {
    pipes: Vec<Vec<Pipe>>,
    width: usize,
    height: usize,
    start: Pos,
}

impl Maze {
    fn solve(&self) -> u32 {
        for assumption in Pipe::ALL {
            let mut length = 0;
            let mut current_pos = self.start;
            let mut last_direction = assumption.directions().unwrap()[0];

            current_pos = current_pos.move_in(last_direction);
            length += 1;

            loop {
                match self.get(current_pos) {
                    None | Some(Pipe::None) => break,
                    Some(Pipe::Start) => return length / 2,
                    Some(current) => {
                        let directions: Vec<_> = current.directions().unwrap().into_iter().filter(|dir| *dir != last_direction.opposite()).collect();
                        if directions.len() == 1 {
                            last_direction = directions[0];
                            current_pos = current_pos.move_in(last_direction);
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
                    start = Some(Pos { row: row as isize, col: col as isize });
                }
                current_row.push(pipe);
            }

            pipes.push(current_row);
        }

        let width = pipes[0].len();
        let height = pipes.len();

        Ok(Maze {
            pipes,
            width,
            height,
            start: start.unwrap(),
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
