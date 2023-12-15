use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/input7");

    println!("Part One: {}", solve_7a(input));
    println!("Part Two: {}", solve_7b(input));
}

fn solve_7a(input: &str) -> u32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| Input::from_str(line).unwrap())
        .collect();
    hands.sort_by(|a, b| {
        b.hand
            .score()
            .cmp(&a.hand.score())
            .then(b.hand.cmp(&a.hand))
    });
    hands
        .into_iter()
        .enumerate()
        .map(|(multiplier, input)| input.bid * (multiplier as u32 + 1))
        .sum()
}

fn solve_7b(input: &str) -> u32 {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| JInput::from_str(line).unwrap())
        .collect();
    hands.sort_by(|a, b| {
        b.hand
            .score()
            .cmp(&a.hand.score())
            .then(b.hand.cmp(&a.hand))
    });
    hands
        .into_iter()
        .enumerate()
        .map(|(multiplier, input)| input.bid * (multiplier as u32 + 1))
        .sum()
}

struct Input {
    hand: Hand,
    bid: u32,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        Ok(Input {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

struct JInput {
    hand: JHand,
    bid: u32,
}

impl FromStr for JInput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        Ok(JInput {
            hand: hand.parse().unwrap(),
            bid: bid.parse().unwrap(),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn score(&self) -> Score {
        let mut copy = self.cards.clone();
        copy.sort();
        let mut counted_cards: Vec<u32> = copy
            .iter()
            .map(|card| (card, 1))
            .group_by(|(card, _)| *card)
            .into_iter()
            .map(|(_, number)| number.map(|(_, freq)| freq).sum())
            .collect();
        counted_cards.sort();

        match counted_cards.pop().unwrap() {
            1 => Score::HighCard,
            2 => match counted_cards.pop().unwrap() {
                2 => Score::TwoPair,
                _ => Score::Pair,
            },
            3 => match counted_cards.pop().unwrap() {
                2 => Score::FullHouse,
                _ => Score::ThreeOfAKind,
            },
            4 => Score::FourOfAKind,
            5 => Score::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            cards: s.chars().map(Card::from_char).collect(),
        })
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct JHand {
    cards: Vec<JCard>,
}

impl JHand {
    fn score(&self) -> Score {
        let mut copy = self.cards.clone();
        copy.sort();
        let mut counted_cards: Vec<u32> = copy
            .iter()
            .map(|card| (card, 1))
            .group_by(|(card, _)| *card)
            .into_iter()
            .filter_map(|(card, number)| {
                if *card != JCard::Joker {
                    Some(number.map(|(_, freq)| freq).sum())
                } else {
                    None
                }
            })
            .collect();
        counted_cards.sort();
        let jokers = copy.iter().filter(|card| **card == JCard::Joker).count();

        match counted_cards.pop().unwrap_or_default() {
            // Only jokers
            0 => Score::FiveOfAKind,
            1 => match jokers {
                0 => Score::HighCard,
                1 => Score::Pair,
                2 => Score::ThreeOfAKind,
                3 => Score::FourOfAKind,
                4 => Score::FiveOfAKind,
                _ => unreachable!(),
            },
            2 => match counted_cards.pop().unwrap_or_default() {
                2 => match jokers {
                    0 => Score::TwoPair,
                    1 => Score::FullHouse,
                    _ => unreachable!(),
                },
                _ => match jokers {
                    0 => Score::Pair,
                    1 => Score::ThreeOfAKind,
                    2 => Score::FourOfAKind,
                    3 => Score::FiveOfAKind,
                    _ => unreachable!(),
                },
            },
            3 => match counted_cards.pop().unwrap_or_default() {
                2 => Score::FullHouse,
                _ => match jokers {
                    0 => Score::ThreeOfAKind,
                    1 => Score::FourOfAKind,
                    2 => Score::FiveOfAKind,
                    _ => unreachable!(),
                },
            },
            4 => match jokers {
                0 => Score::FourOfAKind,
                1 => Score::FiveOfAKind,
                _ => unreachable!(),
            },
            5 => Score::FiveOfAKind,
            _ => unreachable!(),
        }
    }
}

impl FromStr for JHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(JHand {
            cards: s.chars().map(JCard::from_char).collect(),
        })
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Score {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum JCard {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl JCard {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            _ => panic!("invalid card"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_7a;

    #[test]
    fn test_1a() {
        const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        assert_eq!(solve_7a(INPUT), 6440)
    }
}
