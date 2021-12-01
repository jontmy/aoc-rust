pub fn solve_part_one(input: &String) -> i32 {
    let vals = input.lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    vals.iter()
        .zip(vals.iter().skip(1))
        .filter(|(prev, curr)| *curr > *prev)
        .count() as i32
}

pub fn solve_part_two(input: &String) -> i32 {
    let vals = input.lines()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let vals = vals.iter()
        .enumerate()
        .take(vals.len() - 2)
        .map(|(i, c)| c + vals[i + 1] + vals[i + 2])
        .collect::<Vec<i32>>();

    vals.iter()
        .zip(vals.iter().skip(1))
        .filter(|(prev, curr)| *curr > *prev)
        .count() as i32
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