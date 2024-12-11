use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::v2::solver;

pub struct Solver;

impl Solver {
    fn solve(&self, input: &str, iterations: usize) -> usize {
        let mut stones = input.split_whitespace().map(|c| c.to_owned()).counts();
        for _ in 0..iterations {
            stones = stones
                .into_iter()
                .flat_map(|(stone, count)| {
                    if stone == "0" {
                        return vec![("1".to_owned(), count)];
                    }
                    if stone.len() % 2 == 0 {
                        let l = stone[..stone.len() / 2].to_owned();
                        let mut r = (stone[stone.len() / 2..])
                            .trim_start_matches('0')
                            .to_owned();
                        if r.is_empty() {
                            r = "0".to_owned();
                        }
                        return vec![(l, count), (r, count)];
                    }
                    return vec![((stone.parse::<u64>().unwrap() * 2024).to_string(), count)];
                })
                .fold(HashMap::new(), |mut acc, (stone, count)| {
                    *acc.entry(stone).or_insert(0) += count;
                    acc
                });
        }
        stones.into_values().sum()
    }
}

impl solver::Solver<2024, 11> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        self.solve(input, 25)
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        self.solve(input, 75)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::solver::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("125 17", 55312)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }
}
