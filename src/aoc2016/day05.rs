use std::{str::FromStr, fmt::format};

use itertools::Itertools;
use md5::compute;

pub fn solve_part_one(input: String) -> String {
    let input = input.trim();
    println!("{input}");
    (0..).into_iter()
        .map(|i| format!("{:x}", md5::compute(format!("{input}{i}"))))
        .filter(|hash| hash.starts_with("00000"))
        .take(8)
        .map(|hash| hash.chars().nth(5).unwrap())
        .collect()
}

pub fn solve_part_two(input: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case("abc".to_string(), "18f47a30".to_string())]
    fn test_room_is_real(#[case] input: String, #[case] expected: String) {
        assert_eq!(solve_part_one(input), expected)
    }
}