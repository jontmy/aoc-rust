use itertools::Itertools;

pub fn solve_part_one(input: &String) -> usize {
    input.lines()
        .map(|s| s.parse().unwrap())
        .tuple_windows::<(i32, i32)>()
        .filter(|(prev, curr)| *curr > *prev)
        .count()
}

pub fn solve_part_two(input: &String) -> usize {
    input.lines()
        .map(|s| s.parse().unwrap())
        .tuple_windows::<(i32, _, _, i32)>()
        .filter(|(prev, _, _, curr)| *curr > *prev)
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "}.to_string(), 7)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "}.to_string(), 5)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_two(&input))
    }
}