fn main() {
    let input = include_str!("../../inputs/input1ab");

    println!("Part One: {}", solve_1a(input));
    println!("Part Two: {}", solve_1b(input));
}

fn solve_1a(input: &str) -> u32 {
    input.lines().map(find_calibration_value_1a).sum::<u32>()
}

fn solve_1b(input: &str) -> u32 {
    input.lines().map(find_calibration_value_1b).sum::<u32>()
}

fn find_calibration_value_1a(line: &str) -> u32 {
    let digits: Vec<_> = line
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| c as u32 - 48)
        .collect();
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn find_calibration_value_1b(line: &str) -> u32 {
    const PATTERNS: [&str; 18] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut first = None;
    let mut last = None;

    for (index, number) in PATTERNS.into_iter().enumerate() {
        let digit = index % 9 + 1;

        let Some(index) = line.find(number) else {
            continue;
        };

        match first {
            None => first = Some((digit, index)),
            Some((_, first_index)) if index < first_index => first = Some((digit, index)),
            _ => (),
        }

        let Some(index) = line.rfind(number) else {
            continue;
        };

        match last {
            None => last = Some((digit, index)),
            Some((_, last_index)) if index > last_index => last = Some((digit, index)),
            _ => (),
        }
    }

    (first.unwrap().0 * 10 + last.unwrap().0) as _
}

#[cfg(test)]
mod tests {
    use crate::{solve_1a, solve_1b};

    #[test]
    fn test_1a() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve_1a(INPUT), 142)
    }

    #[test]
    fn test_1b() {
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_1b(INPUT), 281)
    }
}
