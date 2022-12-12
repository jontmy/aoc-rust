use std::str::FromStr;

use itertools::Itertools;
use num::Integer;
use scan_fmt::scan_fmt;

use crate::utils::{ self, advent, grid::Grid, coords::Coordinates };

pub struct Solver;

impl Solver {
    pub fn h(c: char) -> usize {
        println!("{c}");
        if c == 'S' {
            return 0;
        } else if c == 'E' {
            return 25;
        }

        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .into_iter()
            .find_position(|d| *d == c)
            .unwrap().0
    }
}

impl advent::Solver<2022, 12> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let grid: Grid<usize> = input
            .trim()
            .lines()
            .map(|l|
                l
                    .chars()
                    .map(|c| Solver::h(c))
                    .collect_vec()
            )
            .collect();

        let start = Coordinates::new(0, 20);
        let end = Coordinates::new(43, 20);

        let path = start
            .generalized_bfs(
                end,
                |c| (0..grid.width()).contains(&c.x()) && (0..grid.height()).contains(&c.y()),
                |c|
                    c
                        .orthogonal_neighbors_bounded(0..grid.width(), 0..grid.height())
                        .into_iter()
                        .filter(
                            |n| grid.get(n.x(), n.y()) <= &(&grid.get(c.x(), c.y()).clone() + 1)
                        )
                        .collect()
            )
            .unwrap();

        path.len()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut ans = vec![];

        for (y, row) in input.lines().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col != 'a' && col != 'S' {
                    continue;
                }
                let start = Coordinates::new(x, y);
                let grid: Grid<usize> = input
                    .trim()
                    .lines()
                    .map(|l|
                        l
                            .chars()
                            .map(|c| Solver::h(c))
                            .collect_vec()
                    )
                    .collect();
                let end = Coordinates::new(43, 20);
                let path = start.generalized_bfs(
                    end,
                    |c| (0..grid.width()).contains(&c.x()) && (0..grid.height()).contains(&c.y()),
                    |c|
                        c
                            .orthogonal_neighbors_bounded(0..grid.width(), 0..grid.height())
                            .into_iter()
                            .filter(
                                |n| grid.get(n.x(), n.y()) <= &(&grid.get(c.x(), c.y()).clone() + 1)
                            )
                            .collect()
                );

                if let Some(path) = path {
                    ans.push(path.len());
                }
            }
        }

        ans.into_iter().min().unwrap()
    }
}