use crate::utils::{advent, grid::Grid};

use itertools::Itertools;

pub struct OldSolver;

impl OldSolver {
    fn parse(input: &str) -> Grid<u32> {
        input
            .trim()
            .lines()
            .map(|x| x.chars().filter_map(|d| d.to_digit(10)).collect_vec())
            .collect::<Grid<_>>()
    }
}

// perf: brute forced
impl advent::OldSolver<2022, 8> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = OldSolver::parse(input);
        let grid = input.clone().shrink();

        let visible_count = grid
            .clone()
            .enumerated_map(|c, h| {
                let (x, y) = (c.x() + 1, c.y() + 1);
                let (left, _, right) = input.row_split_at(x, y);
                let (above, _, below) = input.col_split_at(x, y);
                let dirs = vec![left, right, above, below];

                dirs.into_iter()
                    .map(|dir| dir.into_iter().all(|height| *height < h))
                    .filter(|is_visible| *is_visible)
                    .count()
                    .min(1)
            })
            .into_iter()
            .sum::<usize>();

        visible_count + input.perimeter() // all trees arond the edge of the grid are visible
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let input = OldSolver::parse(input);
        let grid = input.clone().shrink();

        grid.clone()
            .enumerated_map(|c, h| {
                let (left, _, right) = grid.row_split_at(c.x(), c.y());
                let (above, _, below) = grid.col_split_at(c.x(), c.y());
                let dirs = vec![left, right, above, below];

                dirs.into_iter()
                    .map(|dir| {
                        let max_dist = dir.len() + 1; // if this tree is tallest in the direction
                        dir.into_iter()
                            .find_position(|height| **height >= h)
                            .map(|(i, _)| i + 1)
                            .unwrap_or(max_dist)
                    })
                    .product()
            })
            .into_iter()
            .max()
            .unwrap_or(0)
    }
}
