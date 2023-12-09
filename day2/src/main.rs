use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input2ab");

    println!("Part One: {}", solve_1a(input));
    println!("Part Two: {}", solve_1b(input));
}

fn solve_1a(input: &str) -> u32 {
    input
        .lines()
        .map(|game| game.parse().unwrap())
        .filter_map(|game: Game| game.validate(12, 13, 14).ok().map(|_| game.id))
        .sum()
}

fn solve_1b(input: &str) -> u32 {
    input
        .lines()
        .map(|game| game.parse().unwrap())
        .map(|game: Game| {
            game.minimum_color(Color::Red)
                * game.minimum_color(Color::Green)
                * game.minimum_color(Color::Blue)
        })
        .sum()
}

pub struct Game {
    id: u32,
    grabs: Vec<Grab>,
}

impl Game {
    pub fn validate(&self, red: u32, green: u32, blue: u32) -> Result<(), ()> {
        for grab in &self.grabs {
            grab.validate(red, green, blue)?;
        }

        Ok(())
    }

    pub fn minimum_color(&self, color: Color) -> u32 {
        self.grabs
            .iter()
            .map(|grab| grab.minimum_color(color))
            .max()
            .unwrap()
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, grabs) = s.split_once(": ").unwrap();
        let id = game.strip_prefix("Game ").unwrap();
        Ok(Game {
            id: id.parse().unwrap(),
            grabs: grabs
                .split("; ")
                .map(|grab| grab.parse().unwrap())
                .collect(),
        })
    }
}

pub struct Grab {
    amounts: Vec<Amount>,
}

impl Grab {
    pub fn validate(&self, mut red: u32, mut green: u32, mut blue: u32) -> Result<(), ()> {
        for amount in &self.amounts {
            match amount.color {
                Color::Red => red = red.checked_sub(amount.number).ok_or(())?,
                Color::Green => green = green.checked_sub(amount.number).ok_or(())?,
                Color::Blue => blue = blue.checked_sub(amount.number).ok_or(())?,
            };
        }

        Ok(())
    }

    pub fn minimum_color(&self, color: Color) -> u32 {
        self.amounts
            .iter()
            .filter_map(|amount| (amount.color == color).then_some(amount.number))
            .sum()
    }
}

impl FromStr for Grab {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grab {
            amounts: s
                .split(", ")
                .map(|amount| amount.parse().unwrap())
                .collect(),
        })
    }
}

pub struct Amount {
    number: u32,
    color: Color,
}

impl FromStr for Amount {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (number, color) = s.split_once(' ').unwrap();
        Ok(Amount {
            number: number.parse().unwrap(),
            color: color.parse().unwrap(),
        })
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}
