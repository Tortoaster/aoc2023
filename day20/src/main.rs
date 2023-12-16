fn main() {
    let input = include_str!("../../inputs/input20");

    println!("Part One: {}", solve_20a(input));
    println!("Part Two: {}", solve_20b(input));
}

fn solve_20a(_input: &str) -> u32 {
    todo!()
}

fn solve_20b(_input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{solve_20a, solve_20b};

    const INPUT: &str = "";

    #[test]
    fn test_20a() {
        assert_eq!(solve_20a(INPUT), todo!())
    }

    #[test]
    fn test_20b() {
        assert_eq!(solve_20b(INPUT), todo!())
    }
}
