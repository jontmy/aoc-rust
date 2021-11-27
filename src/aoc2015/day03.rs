pub fn solve_part_one(input: &String) -> i32 {
    0
}

pub fn solve_part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::aoc2015::day01::{solve_part_one, solve_part_two};
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(")", 1)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}