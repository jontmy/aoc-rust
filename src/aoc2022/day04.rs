use crate::utils::advent;

use scan_fmt::scan_fmt;

pub struct OldSolver;

impl advent::OldSolver<2022, 4> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .lines()
            .filter_map(|l| scan_fmt!(l, "{d}-{d},{d}-{d}", i32, i32, i32, i32).ok())
            .filter(|(a, b, c, d)| ((a <= c && b >= d) || (c <= a && d >= b)))
            .count()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        input
            .lines()
            .filter_map(|l| scan_fmt!(l, "{d}-{d},{d}-{d}", i32, i32, i32, i32).ok())
            .filter(|(a, b, c, d)| ((a <= c && c <= b) || (c <= a && a <= d)))
            .count()
    }
}
