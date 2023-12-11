use std::{collections::BTreeSet, str::FromStr};

fn main() {
    let input = include_str!("../../inputs/input4");

    println!("Part One: {}", solve_4a(input));
    println!("Part Two: {}", solve_4b(input));
}

fn solve_4a(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|card: Card| card.points())
        .sum()
}

fn solve_4b(input: &str) -> u32 {
    let mut multipliers: Vec<_> = input.lines().map(|_| 1).collect();

    for (index, card) in input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .enumerate()
    {
        for i in 0..card.common() {
            let this_multiplier = multipliers[index];
            if let Some(multiplier) = multipliers.get_mut(index + 1 + i as usize) {
                *multiplier += this_multiplier;
            };
        }
    }

    multipliers.into_iter().sum()
}

pub struct Card {
    number: u32,
    winning: Numbers,
    our: Numbers,
}

impl Card {
    pub fn points(&self) -> u32 {
        match self.common() {
            0 => 0,
            n => 2_u32.pow(n - 1),
        }
    }

    pub fn common(&self) -> u32 {
        let winning: BTreeSet<_> = self.winning.numbers.iter().copied().collect();

        self.our
            .numbers
            .iter()
            .copied()
            .filter(|n| winning.contains(n))
            .count() as u32
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
            our: our.parse().unwrap(),
        })
    }
}

pub struct Numbers {
    numbers: Vec<u32>,
}

impl FromStr for Numbers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Numbers {
            numbers: s.split_whitespace().map(|n| n.parse().unwrap()).collect(),
        })
    }
}
