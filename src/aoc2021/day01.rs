// use std::collections::HashMap;
// use std::fmt::{Debug, Formatter};

// use lazy_static::lazy_static;
// use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    0
}

pub fn solve_part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        123 -> a
    "}.to_string(), 123)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("str", 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}