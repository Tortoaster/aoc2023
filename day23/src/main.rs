fn main() {
    let input = include_str!("../../inputs/input23");

    println!("Part One: {}", solve_23a(input));
    println!("Part Two: {}", solve_23b(input));
}

fn solve_23a(_input: &str) -> u32 {
    todo!()
}

fn solve_23b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_23a, solve_23b};

    const INPUT: &str = "";

    #[test]
    fn test_23a() {
        assert_eq!(solve_23a(INPUT), todo!())
    }

    #[test]
    fn test_23b() {
        assert_eq!(solve_23b(INPUT), todo!())
    }
}
