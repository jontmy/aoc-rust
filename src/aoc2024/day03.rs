use once_cell_regex::regex;

use crate::utils::v2::solver;

pub struct Solver;

impl Solver {
    fn solve(&self, input: &str) -> u64 {
        let re = regex!(r"mul\((\d{1,3}),(\d{1,3})\)");
        re.captures_iter(&input)
            .map(|cap| {
                cap.get(1).unwrap().as_str().parse::<u64>().unwrap()
                    * cap.get(2).unwrap().as_str().parse::<u64>().unwrap()
            })
            .sum()
    }
}

impl solver::Solver<2024, 3> for Solver {
    type Part1 = u64;
    type Part2 = u64;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        self.solve(input)
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let re = regex!(r"(?s)don't\(\).*?(?:do\(\)|$)");
        let input = re.replace_all(input, ".");
        self.solve(&input)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::solver::Solver;
    use rstest::rstest;

    #[rstest]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        161
    )]
    fn test_part_one(#[case] input: &str, #[case] expected: u64) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        48
    )]
    fn test_part_two(#[case] input: &str, #[case] expected: u64) {
        let solver = super::Solver;
        let result = solver.solve_part_two(input);
        assert_eq!(result, expected);
    }
}
