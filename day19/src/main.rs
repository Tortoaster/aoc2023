fn main() {
    let input = include_str!("../../inputs/input19");

    println!("Part One: {}", solve_19a(input));
    println!("Part Two: {}", solve_19b(input));
}

fn solve_19a(_input: &str) -> u32 {
    todo!()
}

fn solve_19b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_19a, solve_19b};

    const INPUT: &str = "";

    #[test]
    fn test_19a() {
        assert_eq!(solve_19a(INPUT), todo!())
    }

    #[test]
    fn test_19b() {
        assert_eq!(solve_19b(INPUT), todo!())
    }
}
