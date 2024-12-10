use itertools::Itertools;
use std::collections::HashSet;

use crate::utils::v2::{
    coords::Coordinates,
    grid::{DenseGrid, GridFind},
    solver,
};

#[derive(Clone, Copy)]
enum AntinodeSpan {
    Limited,
    Unlimited,
}

pub struct Solver;

impl Solver {
    fn compute_antinodes(
        &self,
        grid: &DenseGrid<char, i32>,
        a: Coordinates<i32>,
        b: Coordinates<i32>,
        antinode_span: AntinodeSpan,
    ) -> Vec<Coordinates<i32>> {
        let delta = a - b;
        let da = if a + delta == b { -delta } else { delta };
        let db = if b + delta == a { -delta } else { delta };

        let range = match antinode_span {
            AntinodeSpan::Limited => 1..2,
            AntinodeSpan::Unlimited => 1..i32::MAX,
        };

        let antinodes_a = range
            .clone()
            .map(move |i| a + da * i)
            .take_while(|&coords| grid.is_coords_in_bounds(coords))
            .collect_vec();

        let antinodes_b = range
            .map(move |i| b + db * i)
            .take_while(|&coords| grid.is_coords_in_bounds(coords))
            .collect_vec();

        let mut antinodes = match antinode_span {
            AntinodeSpan::Limited => vec![],
            AntinodeSpan::Unlimited => vec![a, b],
        };
        antinodes.extend(antinodes_a);
        antinodes.extend(antinodes_b);

        antinodes
    }

    fn solve(&self, input: &str, antinode_span: AntinodeSpan) -> usize {
        let chars = input.chars().filter(|c| c.is_alphanumeric()).unique();
        let grid = DenseGrid::<char, i32>::try_from(input).unwrap();
        let mut all_antinodes = HashSet::new();

        for c in chars {
            let nodes = grid.find_all(&c).into_iter().map(Coordinates::from);
            let antinodes = nodes
                .tuple_combinations()
                .flat_map(|(a, b)| self.compute_antinodes(&grid, a, b, antinode_span))
                .filter(|&coords| grid.is_coords_in_bounds(coords));

            all_antinodes.extend(antinodes);
        }

        all_antinodes.len()
    }
}

impl solver::Solver<2024, 8> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        self.solve(input, AntinodeSpan::Limited)
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        self.solve(input, AntinodeSpan::Unlimited)
    }
}
