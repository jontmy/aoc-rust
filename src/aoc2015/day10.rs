use std::iter;

use itertools::Itertools;

pub fn solve_part_one(input: &String) -> i32 {
    let input = input.lines().next().unwrap().to_string();
    rounds(input, 40).len() as i32
}

pub fn solve_part_two(input: &String) -> i32 {
    let input = input.lines().next().unwrap().to_string();
    rounds(input, 50).len() as i32
}

fn next(s: &String) -> String {
    let chars = s.chars().collect::<Vec<char>>();
    let repetitions = s.chars()
        .enumerate()
        .filter(|(i, c)| i.eq(&0) || chars[i - 1] != *c)
        .chain([(s.len(), '_')]) // add a dummy terminator
        .collect_vec();

    repetitions.iter()
        .zip(&mut repetitions.iter().skip(1)) // compare each tuple with the next
        .map(|((start, char), (end, _))| format!("{}{}", end - start, char))
        .collect::<String>()
}

fn rounds(s: String, n: usize) -> String {
    iter::successors(Some(s), |s| Some(next(s)))
        .skip(n)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case("111221", 0)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("str", 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}