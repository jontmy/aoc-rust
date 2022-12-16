use itertools::Itertools;

use crate::utils::{advent, coords::Coordinates, grid::Grid};

pub struct Solver;

impl Solver {
    pub fn elevation(c: char) -> usize {
        match c {
            'S' => 0,
            'E' => 25,
            c => "abcdefghijklmnopqrstuvwxyz".find(c).unwrap(),
        }
    }

    pub fn grid(input: &str) -> Grid<char> {
        input
            .trim()
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect()
    }

    pub fn elevations(input: &str) -> Grid<usize> {
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| Solver::elevation(c)).collect_vec())
            .collect()
    }

    pub fn bfs(
        elevations: &Grid<usize>,
        start: Coordinates<usize>,
        end: Coordinates<usize>,
    ) -> Option<usize> {
        let path = start.generalized_bfs(
            end,
            |c| {
                (0..elevations.width()).contains(&c.x())
                    && (0..elevations.height()).contains(&c.y())
            },
            |c| {
                c.orthogonal_neighbors_bounded(0..elevations.width(), 0..elevations.height())
                    .into_iter()
                    .filter(|n| elevations.get(n.x(), n.y()) <= &(elevations.get(c.x(), c.y()) + 1))
                    .collect()
            },
        );
        path.map(|v| v.len())
    }
}

impl advent::Solver<2022, 12> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let grid = Solver::grid(input);
        let elevations = Solver::elevations(input);
        let start = grid.find('S').unwrap();
        let end = grid.find('E').unwrap();
        Solver::bfs(&elevations, start, end).unwrap()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let grid = Solver::grid(input);
        let elevations = Solver::elevations(input);
        let starts = elevations.find_all(0);
        let end = grid.find('E').unwrap();

        starts
            .into_iter()
            .filter_map(|start| Solver::bfs(&elevations, start, end))
            .min()
            .unwrap()
    }
}
