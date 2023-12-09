use std::collections::BTreeSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input4");

    println!("Part One: {}", solve_4a(input));
    println!("Part Two: {}", solve_4b(input));
}

fn solve_4a(input: &str) -> u32 {
    input.lines().map(|line| line.parse().unwrap()).map(|card: Card| card.points()).sum()
}

fn solve_4b(input: &str) -> u32 {
    todo!()
}

pub struct Card {
    number: u32,
    winning: Numbers,
    our: Numbers,
}

impl Card {
    pub fn points(&self) -> u32 {
        let winning: BTreeSet<_> = self.winning.numbers.iter().copied().collect();
        let common: usize = self.our.numbers.iter().copied().filter(|n| winning.contains(n)).count();
        if common == 0 {
            0
        } else {
            2_u32.pow(common as u32 - 1)
        }
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card").unwrap().trim();
        let (number, s) = s.split_once(": ").unwrap();
        let (winning, our) = s.split_once(" | ").unwrap();

        Ok(Card {
            number: number.parse().unwrap(),
            winning: winning.parse().unwrap(),
            our: our.parse().unwrap()
        })
    }
}

pub struct Numbers {
    numbers: Vec<u32>
}

impl FromStr for Numbers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Numbers { numbers: s.split_whitespace().map(|n| n.parse().unwrap()).collect() })
    }
}