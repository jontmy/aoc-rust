use crate::utils::advent;

use advent_of_code::utils::grid::Grid;
use scan_fmt::scan_fmt;

pub struct Solver;

impl Solver {
    // perf: do this in 1 pass
    pub fn signal_strengths(input: &str) -> Vec<i32> {
        let mut ins = vec![];
        let mut i = 0;
        for line in input.trim().lines() {
            if line.starts_with("noop") {
                i += 1;
                continue;
            }
            let x = scan_fmt!(line, "addx {}", i32).unwrap();
            ins.push((i + 2, x));
            i += 2;
        }
        ins.sort_by_key(|(i, _)| *i);

        let mut xs = vec![1];
        let mut curr = 0;
        for i in 0..240 {
            let x = *xs.last().unwrap();
            if curr >= ins.len() {
                xs.push(x);
                continue;
            }
            let (cycle, inc) = ins[curr];
            if cycle == i {
                xs.push(x + inc);
                curr += 1;
            } else {
                xs.push(x);
            }
        }
        xs
    }
}

impl advent::Solver<2022, 10> for Solver {
    type Part1 = i32;
    type Part2 = String;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        Solver::signal_strengths(input)
            .into_iter()
            .enumerate()
            .skip(20)
            .step_by(40)
            .map(|(cycle, x)| cycle as i32 * x)
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let xs = Solver::signal_strengths(input);
        let grid = Grid::from_generator(6, 40, |c| {
            let (x, y) = c.into();
            let center = xs[y * 40 + x + 1];
            let pixel = if center.abs_diff(x as i32) <= 1 { '#' } else { '.' };
            Some(pixel)
        }, ' ');
        format!("\n{grid}")
    }
}