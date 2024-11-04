use crate::utils::advent;

use itertools::Itertools;

pub struct OldSolver;

impl OldSolver {
    fn find_marker(input: &str, n: usize) -> usize {
        let input = input.trim().chars().collect_vec();
        input
            .windows(n)
            .enumerate()
            .filter(|(_, window)| window.into_iter().all_unique())
            .map(|(i, _)| i + n)
            .next()
            .unwrap()
    }
}

impl advent::OldSolver<2022, 6> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        OldSolver::find_marker(input, 4)
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        OldSolver::find_marker(input, 14)
    }
}
