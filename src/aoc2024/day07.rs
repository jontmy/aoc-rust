use advent_of_code::utils::v2::parser::get_all_ints_unsigned;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::v2::solver;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn exec(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concatenate => {
                let c = format!("{}{}", a, b);
                c.parse().unwrap()
            }
        }
    }
}

pub struct Solver;

impl Solver {
    fn is_equation_possible(&self, equation: &[u64], operators: &[Operator]) -> bool {
        for permutation in operators
            .into_iter()
            .combinations_with_replacement(equation.len() - 2)
            .flat_map(|combintion| {
                combintion
                    .into_iter()
                    .permutations(equation.len() - 2)
                    .unique()
            })
        {
            let (expected, operands) = equation.split_first().unwrap();
            let actual = operands
                .into_iter()
                .skip(1)
                .zip(permutation)
                .fold(operands[0], |acc, (x, op)| op.exec(acc, *x));

            if actual == *expected {
                return true;
            }
        }
        return false;
    }
}

impl solver::Solver<2024, 7> for Solver {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let equations = input.lines().map(get_all_ints_unsigned).collect_vec();

        equations
            .into_par_iter()
            .filter(|equation| {
                self.is_equation_possible(equation, &[Operator::Add, Operator::Multiply])
            })
            .map(|equation| equation[0])
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let equations = input.lines().map(get_all_ints_unsigned).collect_vec();
        let (possible, impossible): (Vec<_>, Vec<_>) =
            equations.into_par_iter().partition(|equation| {
                self.is_equation_possible(equation, &[Operator::Add, Operator::Multiply])
            });

        impossible
            .into_par_iter()
            .filter(|equation| {
                self.is_equation_possible(
                    equation,
                    &[Operator::Add, Operator::Multiply, Operator::Concatenate],
                )
            })
            .chain(possible)
            .map(|equation| equation[0])
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::solver::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("190: 10 19", 190)]
    #[case("3267: 81 40 27", 3267)]
    #[case("83: 17 5", 0)]
    #[case("156: 15 6", 0)]
    #[case("7290: 6 8 6 15", 0)]
    #[case("161011: 16 10 13", 0)]
    #[case("192: 17 8 14", 0)]
    #[case("21037: 9 7 18 13", 0)]
    #[case("292: 11 6 16 20", 292)]
    #[case("4: 1 1 1 1", 4)]
    #[case("8: 2 2 2 2", 8)]
    #[case("10: 2 2 2 2", 10)]
    #[case("16: 2 2 2 2", 16)]
    #[case("1: 1", 1)]
    fn test_part_one(#[case] input: &str, #[case] expected: u64) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("190: 10 19", 190)]
    #[case("3267: 81 40 27", 3267)]
    #[case("83: 17 5", 0)]
    #[case("156: 15 6", 156)]
    #[case("7290: 6 8 6 15", 7290)]
    #[case("161011: 16 10 13", 0)]
    #[case("192: 17 8 14", 192)]
    #[case("21037: 9 7 18 13", 0)]
    #[case("292: 11 6 16 20", 292)]
    #[case("4: 1 1 1 1", 4)]
    #[case("8: 2 2 2 2", 8)]
    #[case("10: 2 2 2 2", 10)]
    #[case("16: 2 2 2 2", 16)]
    #[case("1: 1", 1)]
    fn test_part_two(#[case] input: &str, #[case] expected: u64) {
        let solver = super::Solver;
        let result = solver.solve_part_two(input);
        assert_eq!(result, expected);
    }
}
