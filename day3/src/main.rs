use std::str::FromStr;

fn main() {
    let input = include_str!("../../inputs/input3");

    println!("Part One: {}", solve_3a(input));
    println!("Part Two: {}", solve_3b(input));
}

fn solve_3a(input: &str) -> u32 {
    let lines: Vec<Line> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut part_numbers = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        part_numbers.extend(line.get_part_numbers(
            index.checked_sub(1).and_then(|i| lines.get(i)).cloned(),
            lines.get(index + 1).cloned(),
        ));
    }

    part_numbers
        .into_iter()
        .map(|part_number| part_number.number)
        .sum()
}

fn solve_3b(input: &str) -> u32 {
    let lines: Vec<Line> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut ratios = 0;

    for (index, line) in lines.iter().enumerate() {
        ratios += line.get_ratios(
            index.checked_sub(1).and_then(|i| lines.get(i)).cloned(),
            lines.get(index + 1).cloned(),
        );
    }

    ratios
}

#[derive(Clone, Default)]
struct Line {
    symbols: Vec<Symbol>,
    part_numbers: Vec<PartNumber>,
}

impl Line {
    pub fn get_part_numbers(
        &self,
        previous: Option<Line>,
        next: Option<Line>,
    ) -> impl Iterator<Item = PartNumber> + '_ {
        let mut symbols = self.symbols.clone();
        symbols.extend(previous.unwrap_or_default().symbols);
        symbols.extend(next.unwrap_or_default().symbols);
        symbols.sort();

        self.part_numbers
            .iter()
            .copied()
            .filter(move |part_number| {
                symbols.iter().any(|symbol| symbol.adjacent_to(part_number))
            })
    }

    pub fn get_ratios(&self, previous: Option<Line>, next: Option<Line>) -> u32 {
        let mut ratios = 0;

        let mut numbers = self.part_numbers.clone();
        numbers.extend(previous.unwrap_or_default().part_numbers);
        numbers.extend(next.unwrap_or_default().part_numbers);
        numbers.sort();

        for symbol in &self.symbols {
            if let SymbolType::Gear = symbol.ty {
                let adjacent_numbers: Vec<_> = numbers.iter().filter(|n| symbol.adjacent_to(n)).collect();
                if adjacent_numbers.len() == 2 {
                    ratios += adjacent_numbers[0].number * adjacent_numbers[1].number;
                }
            }
        }

        ratios
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part_numbers = Vec::new();
        let mut symbols = Vec::new();

        let mut number: Option<u32> = None;
        for (index, c) in s.chars().enumerate() {
            match &mut number {
                None => {
                    if c.is_ascii_digit() {
                        number = Some(c as u32 - 48);
                    } else if c != '.' {
                        symbols.push(Symbol { index, ty: SymbolType::from_char(c) })
                    }
                }
                Some(n) => {
                    if c.is_ascii_digit() {
                        *n = *n * 10 + (c as u32 - 48);
                    } else {
                        part_numbers.push(PartNumber {
                            number: *n,
                            index: index - n.to_string().len(),
                        });
                        number = None;
                        if c != '.' {
                            symbols.push(Symbol { index, ty: SymbolType::from_char(c) })
                        }
                    }
                }
            }
        }

        if let Some(n) = number {
            part_numbers.push(PartNumber {
                number: n,
                index: s.len() - n.to_string().len(),
            });
        }

        Ok(Line {
            symbols,
            part_numbers,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Symbol {
    index: usize,
    ty: SymbolType
}

impl Symbol {
    pub fn adjacent_to(&self, part_number: &PartNumber) -> bool {
        part_number.index <= self.index + 1
            && part_number.index + part_number.width() > self.index - 1
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
enum SymbolType {
    Gear,
    Whatever,
}

impl SymbolType {
    pub fn from_char(c: char) -> Self {
        if c == '*' {
            SymbolType::Gear
        } else {
            SymbolType::Whatever
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct PartNumber {
    number: u32,
    index: usize,
}

impl PartNumber {
    pub fn width(&self) -> usize {
        self.number.to_string().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_3a;

    #[test]
    fn test_3a() {
        const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_3a(INPUT), 4361)
    }
}
