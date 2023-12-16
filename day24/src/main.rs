fn main() {
    let input = include_str!("../../inputs/input24");

    println!("Part One: {}", solve_24a(input));
    println!("Part Two: {}", solve_24b(input));
}

fn solve_24a(_input: &str) -> u32 {
    todo!()
}

fn solve_24b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_24a, solve_24b};

    const INPUT: &str = "";

    #[test]
    fn test_24a() {
        assert_eq!(solve_24a(INPUT), todo!())
    }

    #[test]
    fn test_24b() {
        assert_eq!(solve_24b(INPUT), todo!())
    }
}
