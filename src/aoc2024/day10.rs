use itertools::Itertools;

use crate::utils::v2::grid::{DenseGrid, Grid, GridFind, GridSearch};

use crate::utils::v2::solver;

pub struct Solver;

impl solver::Solver<2024, 10> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let grid = DenseGrid::<char, i32>::try_from(input)
            .unwrap()
            .map_into(|c| c.to_string().parse::<i32>().unwrap());

        grid.find_all_iter(&0)
            .map(|start_pos| {
                grid.dfs_find_all(9, start_pos, |(x, y), v| {
                    grid.indexed_adjacent_neighbors_iter(x, y)
                        .filter(|(_, &nv)| nv == v + 1)
                        .map(|(n, _)| n)
                        .collect_vec()
                })
                .len()
            })
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let grid = DenseGrid::<char, i32>::try_from(input)
            .unwrap()
            .map_into(|c| c.to_string().parse::<i32>().unwrap());

        grid.find_all_iter(&0)
            .map(|start_pos| {
                grid.dfs_find_all_with_repeats(9, start_pos, |(x, y), v| {
                    grid.indexed_adjacent_neighbors_iter(x, y)
                        .filter(|(_, &nv)| nv == v + 1)
                        .map(|(n, _)| n)
                        .collect_vec()
                })
                .len()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::solver::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("0123456789", 1)]
    #[case("012345678", 0)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }
}
