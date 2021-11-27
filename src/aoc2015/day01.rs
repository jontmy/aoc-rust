pub fn solve_part_one(input: &String) -> i32 {
    input.trim().chars()
        .map(|x| if x == '(' {1} else {-1})
        .sum()
}

pub fn solve_part_two(input: &String) -> i32 {
    input.trim().chars()
        .map(|x| if x == '(' { 1 } else { -1 })
        .scan(0, |state, x: i32| {
            *state += x;
            Some(*state)
        })
        .position(|x: i32| x == -1)
        .unwrap() as i32 + 1
}

#[cfg(test)]
mod tests {
    use crate::aoc2015::day01::{solve_part_one, solve_part_two};
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}