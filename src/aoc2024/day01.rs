use itertools::Itertools;

use crate::utils::v2::{parser::get_all_ints_signed, solver};

pub struct Solver;

impl Solver {
    fn parse_input(&self, input: &str) -> (Vec<i64>, Vec<i64>) {
        let ints = get_all_ints_signed(input);
        let mut xs = vec![];
        let mut ys = vec![];

        assert!(ints.len() % 2 == 0);
        for i in 0..ints.len() / 2 {
            xs.push(ints[2 * i]);
            ys.push(ints[2 * i + 1]);
        }

        (xs, ys)
    }
}

impl solver::Solver<2024, 1> for Solver {
    type Part1 = u64;
    type Part2 = i64;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let (mut xs, mut ys) = self.parse_input(input);
        xs.sort_unstable();
        ys.sort_unstable();

        xs.into_iter().zip_eq(ys).map(|(x, y)| x.abs_diff(y)).sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let (mut xs, ys) = self.parse_input(input);
        xs.sort_unstable();

        let counts = ys.into_iter().counts();
        xs.into_iter()
            .map(|x| *counts.get(&x).unwrap_or(&0) as i64 * x)
            .sum()
    }
}
