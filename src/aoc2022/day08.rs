use crate::utils::advent;

use itertools::Itertools;

pub struct Solver;

// perf: brute forced
impl advent::Solver<2022, 8> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let s = input
            .trim()
            .lines()
            .map(|x|
                x
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect_vec()
            )
            .collect_vec();

        let h = s.len();
        let w = s.first().unwrap().len();
        let mut total = h + h + w - 2 + w - 2; // + perimeter
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let c = s[y][x];
                let mut d = (true, true, true, true);
                for i in 0..y {
                    // up
                    if s[i][x] >= c {
                        d.0 = false;
                    }
                }
                for i in y + 1..h {
                    // down
                    if s[i][x] >= c {
                        d.1 = false;
                    }
                }
                for i in 0..x {
                    // left
                    if s[y][i] >= c {
                        d.2 = false;
                    }
                }
                for i in x + 1..w {
                    // right
                    if s[y][i] >= c {
                        d.3 = false;
                    }
                }
                total += if d.0 || d.1 || d.2 || d.3 { 1 } else { 0 };
            }
        }
        total
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let s = input
            .trim()
            .lines()
            .map(|x|
                x
                    .chars()
                    .map(|d| d.to_digit(10).unwrap())
                    .collect_vec()
            )
            .collect_vec();

        let h = s.len();
        let w = s.first().unwrap().len();
        let mut max = 0;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let c = s[y][x];
                let mut d = (0, 0, 0, 0);
                for i in (0..y).rev() {
                    // up
                    d.0 += 1;
                    if s[i][x] >= c {
                        break;
                    }
                }
                for i in y + 1..h {
                    // down
                    d.1 += 1;
                    if s[i][x] >= c {
                        break;
                    }
                }
                for i in (0..x).rev() {
                    // left
                    d.2 += 1;
                    if s[y][i] >= c {
                        break;
                    }
                }
                for i in x + 1..w {
                    // right
                    d.3 += 1;
                    if s[y][i] >= c {
                        break;
                    }
                }
                max = max.max(d.0 * d.1 * d.2 * d.3);
            }
        }
        max
    }
}