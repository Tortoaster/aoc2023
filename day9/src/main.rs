use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input9a");

    println!("Part One: {}", solve_9a(input));
    println!("Part Two: {}", solve_9b(input));
}

fn solve_9a(input: &str) -> i32 {
    input.lines()
        .map(|line| line.parse().unwrap())
        .map(|seq: Sequence| seq.extrapolate())
        .sum()
}

fn solve_9b(input: &str) -> i32 {
    input.lines()
        .map(|line| line.parse().unwrap())
        .map(|seq: Sequence| seq.extrapolate_before())
        .sum()
}

#[derive(Clone)]
pub struct Sequence {
    numbers: Vec<i32>,
}

impl Sequence {
    pub fn is_zero(&self) -> bool {
        self.numbers.iter().all(|n| *n == 0)
    }

    fn derivative(&self) -> Sequence {
        let mut offset = self.numbers.clone();
        let _ = offset.remove(0);
        for (index, n) in offset.iter_mut().enumerate() {
            *n -= self.numbers[index];
        }
        Sequence {
            numbers: offset
        }
    }

    pub fn extrapolate(&self) -> i32 {
        let mut derivatives = Vec::new();
        let mut current = self.clone();
        derivatives.push(current.clone());
        loop {
            current = current.derivative();
            derivatives.push(current.clone());
            if current.is_zero() {
                break;
            }
        }

        let mut extrapolated_value = 0;
        loop {
            derivatives.pop();
            match derivatives.last() {
                None => return extrapolated_value,
                Some(last) => extrapolated_value += last.numbers.last().unwrap(),
            }
        }
    }

    pub fn extrapolate_before(&self) -> i32 {
        let mut derivatives = Vec::new();
        let mut current = self.clone();
        derivatives.push(current.clone());
        loop {
            current = current.derivative();
            derivatives.push(current.clone());
            if current.is_zero() {
                break;
            }
        }

        let mut extrapolated_value = 0;
        loop {
            derivatives.pop();
            match derivatives.last() {
                None => return extrapolated_value,
                Some(last) => extrapolated_value = last.numbers.first().unwrap() - extrapolated_value,
            }
        }
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            numbers: s.split(' ').map(|number| number.parse().unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_9a};

    #[test]
    fn test_1a() {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(solve_9a(INPUT), 114)
    }
}
