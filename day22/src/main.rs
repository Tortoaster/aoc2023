fn main() {
    let input = include_str!("../../inputs/input17");

    println!("Part One: {}", solve_22a(input));
    println!("Part Two: {}", solve_22b(input));
}

fn solve_22a(_input: &str) -> u32 {
    todo!()
}

fn solve_22b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_22a, solve_22b};

    const INPUT: &str = "";

    #[test]
    fn test_22a() {
        assert_eq!(solve_22a(INPUT), todo!())
    }

    #[test]
    fn test_22b() {
        assert_eq!(solve_22b(INPUT), todo!())
    }
}
