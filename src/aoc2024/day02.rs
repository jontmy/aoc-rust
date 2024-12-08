use crate::utils::v2::parser::get_all_ints_signed;
use itertools::Itertools;

use crate::utils::v2::solver;

pub struct Solver;

impl Solver {
    fn is_safe(&self, line: &[i64]) -> bool {
        let mut deltas = line
            .into_iter()
            .tuple_windows()
            .map(|(l, r)| l - r)
            .peekable();

        let signum = match deltas.peek().map(|d| d.signum()) {
            None => return true,
            Some(0) => return false,
            Some(x) => x,
        };
        deltas.all(|d| d.signum() == signum && 1 <= d.abs() && d.abs() <= 3)
    }
}

impl solver::Solver<2024, 2> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .lines()
            .map(get_all_ints_signed)
            .filter(|l| self.is_safe(l))
            .count()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut result = 0;
        for line in input.lines().map(get_all_ints_signed) {
            for i in 0..line.len() {
                let mut line = line.clone();
                line.remove(i);
                if self.is_safe(&line) {
                    result += 1;
                    break;
                }
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
    #[case("1", 1)]
    #[case("1 1", 0)]
    #[case("1 1 1", 0)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("1", 1)]
    #[case("1 1", 1)]
    #[case("1 1 1", 0)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let solver = super::Solver;
        let result = solver.solve_part_two(input);
        assert_eq!(result, expected);
    }
}
