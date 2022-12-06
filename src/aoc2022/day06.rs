use std::collections::HashSet;

use crate::utils::advent;

use itertools::Itertools;
use scan_fmt::scan_fmt;

pub struct Solver;

impl advent::Solver<2022, 6> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = input.trim();
        for i in 0..input.len() - 4 {
            if input[i..i + 4].chars().unique().count() == 4 {
                return i + 4;
            }
        }
        unreachable!()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let input = input.trim();
        for i in 0..input.len() - 14 {
            if input[i..i + 14].chars().unique().count() == 14 {
                return i + 14;
            }
        }
        unreachable!()
    }
}