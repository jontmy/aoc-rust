use itertools::Itertools;

use crate::utils::advent;

pub struct Solver;

impl advent::Solver<2022, 1> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .trim()
            .split("\n\n")
            .map(|l|
                l
                    .split("\n")
                    .into_iter()
                    .map(|c| c.parse::<i32>().unwrap())
                    .sum()
            )
            .max()
            .unwrap()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        input
            .trim()
            .split("\n\n")
            .map(|l|
                l
                    .split("\n")
                    .into_iter()
                    .map(|c| c.parse::<i32>().unwrap())
                    .sum::<i32>()
            )
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
    }
}