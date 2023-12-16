fn main() {
    let input = include_str!("../../inputs/input18");

    println!("Part One: {}", solve_18a(input));
    println!("Part Two: {}", solve_18b(input));
}

fn solve_18a(_input: &str) -> u32 {
    todo!()
}

fn solve_18b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_18a, solve_18b};

    const INPUT: &str = "";

    #[test]
    fn test_18a() {
        assert_eq!(solve_18a(INPUT), todo!())
    }

    #[test]
    fn test_18b() {
        assert_eq!(solve_18b(INPUT), todo!())
    }
}
