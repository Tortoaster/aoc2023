use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../../inputs/input8");

    println!("Part One: {}", solve_8a(input));
    println!("Part Two: {}", solve_8b(input));
}

fn solve_8a(input: &str) -> u32 {
    let input = Input::from_str(input);

    let mut current = &input.locations[Input::START];
    let mut step_index = 0;
    let mut steps = 1;

    loop {
        let direction = input.directions[step_index];
        let new_location = current.locations[&direction];

        if new_location == Input::END {
            return steps;
        }

        current = &input.locations[new_location];
        step_index = (step_index + 1) % input.directions.len();
        steps += 1;
    }
}

fn solve_8b(input: &str) -> u32 {
    let input = Input::from_str(input);

    let mut current: Vec<_> = input
        .locations
        .iter()
        .filter_map(|(name, location)| name.ends_with('A').then_some(location))
        .collect();
    let mut step_index = 0;
    let mut steps = 1;

    loop {
        let direction = input.directions[step_index];
        let new_locations: Vec<_> = current
            .iter()
            .map(|location| location.locations[&direction])
            .collect();

        if new_locations.iter().all(|name| name.ends_with('Z')) {
            return steps;
        }

        for (index, location) in current.iter_mut().enumerate() {
            *location = &input.locations[new_locations[index]];
        }
        step_index = (step_index + 1) % input.directions.len();
        steps += 1;
    }
}

struct Input<'a> {
    directions: Vec<Direction>,
    locations: BTreeMap<&'a str, Location<'a>>,
}

impl<'a> Input<'a> {
    const START: &'static str = "AAA";
    const END: &'static str = "ZZZ";

    fn from_str(input: &'a str) -> Self {
        let (directions, locations) = input.split_once("\n\n").unwrap();

        let directions = directions.chars().map(Direction::from_char).collect();
        let locations = locations.lines().map(Location::from_str).collect();

        Input {
            directions,
            locations,
        }
    }
}

struct Location<'a> {
    locations: BTreeMap<Direction, &'a str>,
}

impl<'a> Location<'a> {
    fn from_str(input: &'a str) -> (&'a str, Self) {
        let (name, location) = input.split_once(" = ").unwrap();

        let location = location.strip_prefix('(').unwrap();
        let location = location.strip_suffix(')').unwrap();
        let (left, right) = location.split_once(", ").unwrap();

        let mut locations = BTreeMap::new();
        locations.insert(Direction::Left, left);
        locations.insert(Direction::Right, right);

        (name, Self { locations })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid direction"),
        }
    }
}
