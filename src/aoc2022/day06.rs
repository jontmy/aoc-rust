use crate::utils::advent;

use itertools::Itertools;

pub struct Solver;

impl Solver {
    fn find_marker(input: &str, n: usize) -> usize {
        let input = input.trim().chars().collect_vec();
        input.windows(n).enumerate()
            .filter(|(_, window)| window.into_iter().all_unique())
            .map(|(i, _)| i + n)
            .next().unwrap()
    }
}

impl advent::Solver<2022, 6> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        Solver::find_marker(input, 4)
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        Solver::find_marker(input, 14)
    }
}