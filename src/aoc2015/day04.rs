use md5;

pub fn solve_part_one(input: &String) -> i32 {
    (0..u32::MAX)
        .map(|i| md5::compute(format!("{}{}", input, i)))
        .map(|hash| format!("{:x}", hash))
        .enumerate()
        .find(|p| p.1.starts_with("00000"))
        .unwrap().0 as i32
}

pub fn solve_part_two(input: &String) -> i32 {
    (0..u32::MAX)
        .map(|i| md5::compute(format!("{}{}", input, i)))
        .map(|hash| format!("{:x}", hash))
        .enumerate()
        .find(|p| p.1.starts_with("000000"))
        .unwrap().0 as i32
}

#[cfg(test)]
mod tests {
    use crate::aoc2015::day04::{solve_part_one};
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }
}