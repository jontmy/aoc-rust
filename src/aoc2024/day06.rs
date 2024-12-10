use std::collections::HashSet;

use crate::utils::v2::grid::{DenseGrid, Grid, GridFind};
use crate::utils::v2::solver;

pub struct Solver;

impl Solver {
    fn parse_input(&self, input: &str) -> (DenseGrid<char, i32>, HashSet<(i32, i32)>, (i32, i32)) {
        let mut grid = DenseGrid::try_from(input).unwrap();
        let mut visited = HashSet::new();

        let initial = grid.find(&'^').unwrap();
        let (mut x, mut y) = initial;
        grid.set(x, y, '.');
        visited.insert((x, y));

        let (mut dx, mut dy) = (-1, 0);
        while let Some(&c) = grid.get(x as i32 + dx, y as i32 + dy) {
            if c == '#' {
                (dx, dy) = (dy, -dx);
            } else {
                visited.insert((x, y));
                x = x + dx;
                y = y + dy;
            }
        }

        visited.insert((x, y));
        (grid, visited, initial)
    }
}

impl solver::Solver<2024, 6> for Solver {
    type Part1 = usize;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let (_, visited, _) = self.parse_input(input);
        visited.len()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let (mut grid, mut visited, initial) = self.parse_input(input);
        visited.remove(&initial);
        let mut result = 0;
        let abort_threshold = visited.len() * 2; // FIXME: check for infinite loops properly

        for (sx, sy) in visited {
            assert!(grid.get(sx, sy) == Some(&'.'));
            grid.set(sx, sy, '#');

            let mut visited_pool = HashSet::new();
            let (mut x, mut y) = initial;
            let (mut dx, mut dy) = (-1, 0);

            let mut step_count = 0;
            while let Some(&c) = grid.get(x as i32 + dx, y as i32 + dy) {
                step_count += 1;
                if c == '#' {
                    (dx, dy) = (dy, -dx);
                } else {
                    visited_pool.insert((x, y));
                    x = x + dx;
                    y = y + dy;
                }
                if step_count > abort_threshold {
                    result += 1;
                    break;
                }
            }
            grid.set(sx, sy, '.');
        }

        result
    }
}
