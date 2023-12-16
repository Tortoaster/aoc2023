fn main() {
    let input = include_str!("../../inputs/input21");

    println!("Part One: {}", solve_21a(input));
    println!("Part Two: {}", solve_21b(input));
}

fn solve_21a(_input: &str) -> u32 {
    todo!()
}

fn solve_21b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_21a, solve_21b};

    const INPUT: &str = "";

    #[test]
    fn test_21a() {
        assert_eq!(solve_21a(INPUT), todo!())
    }

    #[test]
    fn test_21b() {
        assert_eq!(solve_21b(INPUT), todo!())
    }
}
