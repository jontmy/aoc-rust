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

    use super::{next};

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_part_one(#[case] input: String, #[case] expected: &str) {
        assert_eq!(expected, next(&input))
    }
}