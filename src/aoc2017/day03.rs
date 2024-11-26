use crate::utils::v2::{
    aoc,
    sparse_grid::{Grid, SparseGrid},
};

pub struct Solver;

impl aoc::Solver<2017, 3> for Solver {
    type Part1 = u32;
    type Part2 = u32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = input.parse::<u32>().unwrap();
        let mut ring_diameter = (input as f64).sqrt().ceil() as u32;
        if (ring_diameter % 2) == 0 {
            ring_diameter += 1;
        }
        assert!(ring_diameter % 2 == 1);
        let ring_num = ring_diameter / 2;
        for i in 0..4 {
            let edge_max = ring_diameter * ring_diameter - i * (ring_diameter - 1);
            let edge_min = edge_max - ring_diameter + 1;
            assert!(input <= edge_max);
            if input < edge_min {
                continue;
            }
            let edge_mid = edge_max - ring_num;
            return edge_mid.abs_diff(input) + ring_num;
        }
        unreachable!();
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let input = input.parse::<u32>().unwrap();
        let mut ring_diameter = (input as f64).sqrt().ceil() as u32;
        if (ring_diameter % 2) == 0 {
            ring_diameter += 1;
        }
        assert!(ring_diameter % 2 == 1);

        let mut grid: SparseGrid<u32> = SparseGrid::new();
        grid.insert(0, 0, 1);

        let mut x = 0;
        let mut y = 0;
        for d in 1.. {
            for _ in 0..(2 * d - 1) {
                x += 1;
                let value = grid.get_and_set_next_value(x, y);
                if value > input {
                    return value;
                }
            }
            for _ in 0..(2 * d - 1) {
                y += 1;
                let value = grid.get_and_set_next_value(x, y);
                if value > input {
                    return value;
                }
            }
            for _ in 0..(2 * d) {
                x -= 1;
                let value = grid.get_and_set_next_value(x, y);
                if value > input {
                    return value;
                }
            }
            for _ in 0..(2 * d) {
                y -= 1;
                let value = grid.get_and_set_next_value(x, y);
                if value > input {
                    return value;
                }
            }
        }
        unreachable!();
    }
}

impl SparseGrid<u32> {
    fn get_and_set_next_value(&mut self, x: i32, y: i32) -> u32 {
        let value = self.all_neighbors(x, y).into_iter().sum();
        self.insert(x, y, value);
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::aoc::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("1", "0")]
    #[case("12", "3")]
    #[case("23", "2")]
    #[case("1024", "31")]
    fn test_part_one(#[case] input: String, #[case] expected: &str) {
        let result = super::Solver.solve_part_one(&input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("1", "2")]
    #[case("2", "4")]
    #[case("4", "5")]
    #[case("5", "10")]
    #[case("10", "11")]
    #[case("11", "23")]
    #[case("23", "25")]
    #[case("747", "806")]
    fn test_part_two(#[case] input: &str, #[case] expected: &str) {
        let solver = super::Solver;
        let result = solver.solve_part_two(input);
        assert_eq!(result, expected);
    }
}
