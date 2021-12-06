use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::iter;

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;

pub fn solve_part_one(input: &String) -> i64 {
    fish_after_n_days(input, 80)
}

    fn fish_after_n_days(input: &String, n: usize) -> i64 {
        let fish = input.trim()
            .split(',').into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .fold(HashMap::new(), |mut freqs, value| -> HashMap<i32, i64> {
                *freqs.entry(value).or_insert(0) += 1;
                freqs
            });

        let fish = (0..=8).into_iter()
            .map(|days| *fish.get(&days).or(Some(&0)).unwrap())
            .collect::<VecDeque<_>>();

        (0..n).into_iter()
            .fold(fish, |mut fish, _| {
                fish.rotate_left(1);
                fish[6] += fish[8];
                fish
            })
            .into_iter()
            .sum()
    }

pub fn solve_part_two(input: &String) -> i64 {
    fish_after_n_days(input, 256)
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        3,4,3,1,2
    "}.to_string(), 5934)]
    fn test_part_one(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        3,4,3,1,2
    "}.to_string(), 26984457539)]
    fn test_part_two(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, solve_part_two(&input))
    }
}