use itertools::Itertools;

use crate::utils::{ advent, grid::Grid, coords::Coordinates };

pub struct Solver;

impl Solver {
    pub fn elevation(c: char) -> usize {
        match c {
            'S' => 0,
            'E' => 25,
            c => "abcdefghijklmnopqrstuvwxyz".find(c).unwrap(),
        }
    }

    pub fn grid(input: &str) -> Grid<usize> {
        input
            .trim()
            .lines()
            .map(|l|
                l
                    .chars()
                    .map(|c| Solver::elevation(c))
                    .collect_vec()
            )
            .collect()
    }

    pub fn bfs(input: &str, start: Coordinates<usize>, end: Coordinates<usize>) -> Option<usize> {
        let grid = Solver::grid(input);
        let path = start.generalized_bfs(
            end,
            |c| (0..grid.width()).contains(&c.x()) && (0..grid.height()).contains(&c.y()),
            |c|
                c
                    .orthogonal_neighbors_bounded(0..grid.width(), 0..grid.height())
                    .into_iter()
                    .filter(|n| grid.get(n.x(), n.y()) <= &(&grid.get(c.x(), c.y()).clone() + 1))
                    .collect()
        );
        path.map(|v| v.len())
    }
}

impl advent::Solver<2022, 12> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let start = Coordinates::new(0, 20);
        let end = Coordinates::new(43, 20);
        Solver::bfs(input, start, end).unwrap()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut ans = vec![];
        for (y, row) in input.lines().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col != 'a' && col != 'S' {
                    continue;
                }
                let start = Coordinates::new(x, y);
                let end = Coordinates::new(43, 20);
                let len = Solver::bfs(input, start, end);
                ans.push(len);
            }
        }
        ans.into_iter().flatten().min().unwrap()
    }
}