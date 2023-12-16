fn main() {
    let input = include_str!("../../inputs/input17");

    println!("Part One: {}", solve_17a(input));
    println!("Part Two: {}", solve_17b(input));
}

fn solve_17a(_input: &str) -> u32 {
    todo!()
}

fn solve_17b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_17a, solve_17b};

    const INPUT: &str = "";

    #[test]
    fn test_17a() {
        assert_eq!(solve_17a(INPUT), todo!())
    }

    #[test]
    fn test_17b() {
        assert_eq!(solve_17b(INPUT), todo!())
    }
}
