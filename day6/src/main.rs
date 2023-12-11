fn main() {
    let input = include_str!("../../inputs/input6");

    println!("Part One: {}", solve_6a(input));
    println!("Part Two: {}", solve_6b(input));
}

fn solve_6a(input: &str) -> u64 {
    parse(input)
        .map(|(time_limit, record_distance)| solve(time_limit as u64, record_distance as u64))
        .product()
}

fn solve_6b(input: &str) -> u64 {
    let (time_limit, record_distance) = parse_as_one(input);
    solve_efficiently(time_limit, record_distance)
}

fn solve(time_limit: u64, record_distance: u64) -> u64 {
    let mut possibilities = 0;

    for time in 0..time_limit {
        let distance = time * (time_limit - time);
        if distance > record_distance {
            possibilities += 1;
        }
    }

    possibilities
}

fn solve_efficiently(time_limit: u64, record_distance: u64) -> u64 {
    let temp1 = (time_limit as f64
        + (time_limit as f64 * time_limit as f64 - 4.0 * record_distance as f64).sqrt())
        / 2.0;
    let temp2 = (time_limit as f64
        - (time_limit as f64 * time_limit as f64 - 4.0 * record_distance as f64).sqrt())
        / 2.0;
    let right = temp1.max(temp2);
    let left = temp1.min(temp2);
    assert!(left < right);
    let left_clamped = left.clamp(0.0, time_limit as f64);
    let right_clamped = right.clamp(0.0, time_limit as f64);

    let min = if left_clamped.fract() == 0.0 {
        left_clamped + 1.0
    } else {
        left_clamped.ceil()
    };
    let max = if right_clamped.fract() == 0.0 {
        right_clamped - 1.0
    } else {
        right_clamped.floor()
    };
    if max >= min {
        (max - min + 1.0) as u64
    } else {
        0
    }
}

fn parse(s: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    let mut lines = s.lines();

    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap());

    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap());

    times.zip(distances)
}

fn parse_as_one(s: &str) -> (u64, u64) {
    let mut lines = s.lines();

    let times = lines.next().unwrap().strip_prefix("Time: ").unwrap();
    let time = times
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u64 - 48)
        .fold(0, |acc, n| acc * 10 + n);

    let distances = lines.next().unwrap().strip_prefix("Distance: ").unwrap();
    let distance = distances
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u64 - 48)
        .fold(0, |acc, n| acc * 10 + n);

    (time, distance)
}

#[cfg(test)]
mod tests {
    use crate::solve_6a;

    #[test]
    fn test_6a() {
        const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(solve_6a(INPUT), 288)
    }
}
