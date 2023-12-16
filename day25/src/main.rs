fn main() {
    let input = include_str!("../../inputs/input25");

    println!("Part One: {}", solve_25a(input));
    println!("Part Two: {}", solve_25b(input));
}

fn solve_25a(_input: &str) -> u32 {
    todo!()
}

fn solve_25b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_25a, solve_25b};

    const INPUT: &str = "";

    #[test]
    fn test_25a() {
        assert_eq!(solve_25a(INPUT), todo!())
    }

    #[test]
    fn test_25b() {
        assert_eq!(solve_25b(INPUT), todo!())
    }
}
