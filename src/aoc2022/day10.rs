
use crate::utils::advent;

use advent_of_code::utils::grid::Grid;
use scan_fmt::scan_fmt;

pub struct Solver;

impl advent::Solver<2022, 10> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = input.trim().lines();

        let mut ins = vec![];

        let mut i = 0;
        for (_, line) in input.enumerate() {
            if line.starts_with("noop") {
                i += 1;
                continue;
            } else if line.starts_with("addx") {
                let x = scan_fmt!(line, "addx {}", i32).unwrap();
                ins.push(("addx", i + 2, x));

                i += 2;
            } else {
                panic!();
            }
        }
        ins.sort_by_key(|(_, i, _)| *i);
        ins.push(("addx", 999, 0));

        for x in &ins {
            println!("{:?}", x);
        }

        let mut ans = vec![1];

        let mut curr = 0;
        for i in 0..220 {
            let (_, cycle, x) = ins[curr];
            if cycle < i {
                panic!("{i}, {cycle}");
                // ans.push(*ans.last().unwrap())
            }
            if cycle == i {
                ans.push(*ans.last().unwrap() + x);
                curr += 1;
            } else {
                ans.push(*ans.last().unwrap());
            }
        }

        vec![20, 60, 100, 140, 180, 220]
            .into_iter()
            .map(|i| (i as i32) * ans[i])
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let input = input.trim().lines();

        let mut ins = vec![];

        let mut i = 0;
        for (_, line) in input.enumerate() {
            if line.starts_with("noop") {
                i += 1;
                continue;
            } else if line.starts_with("addx") {
                let x = scan_fmt!(line, "addx {}", i32).unwrap();
                ins.push(("addx", i + 2, x));

                i += 2;
            } else {
                panic!();
            }
        }
        ins.sort_by_key(|(_, i, _)| *i);
        let mut ans = vec![1];
        let mut curr = 0;
        for i in 0..220 {
            let (_, cycle, x) = ins[curr];
            if cycle < i {
                panic!("{i}, {cycle}");
            }
            if cycle == i {
                ans.push(*ans.last().unwrap() + x);
                curr += 1;
            } else {
                ans.push(*ans.last().unwrap());
            }
        }

        println!("{:?}", ans);

        let mut grid = Grid::from_value(6, 40, ' ');
        for i in 0..6 {
            for j in 0..40 {
                let x = i * 40 + j + 1;
                let x = x as usize;

                let row = i as usize;
                if x >= ans.len() {
                    grid.set(j, row, '.');
                    continue;
                }

                if ans[x] < 0 {
                    grid.set(j, row, '.');
                    continue;
                }

                let col = ans[x] as usize;
                if col.abs_diff(j) <= 1 {
                    grid.set(j, row, '#');
                } else {
                    grid.set(j, row, '.');
                }
                println!("{grid}");
            }
        }
        println!("{grid}");

        0
    }
}