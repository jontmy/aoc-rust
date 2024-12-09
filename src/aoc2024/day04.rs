use anyhow::{anyhow, Result};

use crate::utils::v2::{grid::DenseGrid, solver};

pub struct Solver;

impl solver::Solver<2024, 4> for Solver {
    type Part1 = Result<i32>;
    type Part2 = Result<i32>;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let grid = DenseGrid::try_from(input)?;
        dbg!(&grid);
        Err(anyhow!("Not implemented"))
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        Err(anyhow!("Not implemented"))
    }
}
