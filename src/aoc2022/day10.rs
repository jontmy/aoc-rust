use std::iter;

use crate::utils::{self, v2::solver, grid::Grid};

pub struct Solver;

impl Solver {
    pub fn signal_strengths(input: &str) -> Vec<i32> {
        ("noop\nnoop\n".to_owned() + input)
            .trim()
            .lines()
            .map(|l| utils::get_all_nums::<i32>(l))
            .flat_map(|xs| iter::once(0).chain(xs))
            .scan(1, |f, x| {
                *f += x;
                Some(*f)
            })
            .collect()
    }
}

impl solver::Solver<2022, 10> for Solver {
    type Part1 = i32;
    type Part2 = String;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        Solver::signal_strengths(input)
            .into_iter()
            .enumerate()
            .skip(20)
            .step_by(40)
            .map(|(cycle, x)| (cycle as i32) * x)
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let xs = Solver::signal_strengths(input);
        let grid = Grid::from_generator(
            6,
            40,
            |c| {
                let (x, y) = c.into();
                let center = xs[y * 40 + x + 1];
                let pixel = if center.abs_diff(x as i32) <= 1 {
                    '#'
                } else {
                    ' '
                };
                Some(pixel)
            },
            ' ',
        );
        utils::ocr(&grid)
    }
}
