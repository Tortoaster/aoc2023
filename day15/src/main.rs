use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input15");

    println!("Part One: {}", solve_14a(input));
    println!("Part Two: {}", solve_14b(input));
}

fn solve_14a(input: &str) -> u32 {
    input.trim().split(',').map(hash).sum()
}

fn solve_14b(input: &str) -> u32 {
    let ops = input
        .trim()
        .split(',')
        .map(|op| Operation::from_str(op).unwrap());
    let mut map = HashMap::new();

    for op in ops {
        match op {
            Operation::Insert(key, value) => map.insert(key, value),
            Operation::Remove(key) => map.remove(&key),
        }
    }

    map.focusing_power()
}

enum Operation {
    Insert(String, u32),
    Remove(String),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('=') {
            None => {
                let key = s.strip_suffix('-').unwrap();
                Ok(Operation::Remove(key.to_owned()))
            }
            Some((key, value)) => {
                let value = value.parse().unwrap();
                Ok(Operation::Insert(key.to_owned(), value))
            }
        }
    }
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

struct HashMap {
    boxes: [Vec<(String, u32)>; 256],
}

impl HashMap {
    fn new() -> Self {
        Self {
            boxes: [(); 256].map(|_| Vec::new()),
        }
    }

    fn focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .filter(|(_, current_box)| !current_box.is_empty())
            .map(|(index, current_box)| {
                (index + 1) as u32
                    * current_box
                        .iter()
                        .enumerate()
                        .map(|(index, (_, value))| (index + 1) as u32 * *value)
                        .sum::<u32>()
            })
            .sum()
    }

    fn insert(&mut self, key: String, value: u32) {
        let hash = hash(&key);
        let current_box = &mut self.boxes[hash as usize];

        match current_box
            .iter_mut()
            .find_map(|(k, v)| (*k == key).then_some(v))
        {
            None => current_box.push((key, value)),
            Some(old_value) => *old_value = value,
        }
    }

    fn remove(&mut self, key: &str) {
        let hash = hash(key);
        let current_box = &mut self.boxes[hash as usize];

        if let Some(index) = current_box.iter().position(|(k, _)| *k == key) {
            current_box.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_14a, solve_14b};

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_14a() {
        assert_eq!(solve_14a(INPUT), 1320)
    }

    #[test]
    fn test_14b() {
        assert_eq!(solve_14b(INPUT), 145)
    }
}
