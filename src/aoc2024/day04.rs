use crate::utils::v2::{
    grid::{DenseGrid, Grid},
    solver,
};

const D: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

pub struct Solver;

impl solver::Solver<2024, 4> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let grid = DenseGrid::try_from(input).unwrap();
        let mut result = 0;
        for ((x, y), c) in grid.as_ndarray().indexed_iter() {
            if *c != 'X' {
                continue;
            }
            for (dx, dy) in D {
                for (i, c) in "MAS".chars().enumerate() {
                    if let Some(v) = grid.get(
                        x as i32 + dx * (i + 1) as i32,
                        y as i32 + dy * (i + 1) as i32,
                    ) {
                        if *v != c {
                            break;
                        }
                        if i == 2 {
                            result += 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        result
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let grid = DenseGrid::try_from(input).unwrap();
        let mut result = 0;

        for ((x, y), c) in grid.as_ndarray().indexed_iter() {
            if *c != 'A' {
                continue;
            }

            let (ms, ss): (Vec<_>, Vec<_>) = grid
                .indexed_diagonal_neighbors_iter(x, y)
                .filter(|(_, c)| **c == 'M' || **c == 'S')
                .partition(|(_, c)| **c == 'M');

            if ms.len() != 2 || ss.len() != 2 {
                continue;
            }
            let ((m1, _), (m2, _)) = (ms[0], ms[1]);
            let ((s1, _), (s2, _)) = (ss[0], ss[1]);

            if (m1.0 == m2.0 && s1.0 == s2.0) || (m1.1 == m2.1 && s1.1 == s2.1) {
                result += 1
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::solver::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("XMAS", 1)]
    #[case("XMASAMX", 2)]
    fn test_part_one(#[case] input: &str, #[case] expected: i32) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }
}
