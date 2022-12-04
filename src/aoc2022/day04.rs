use itertools::Itertools;
use once_cell_regex::regex;

use crate::utils::advent;

pub struct Solver;

impl advent::Solver<2022, 4> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let regex = regex!(r"(\d+)-(\d+),(\d+)-(\d+)");
        let mut count = 0;
        for line in input.lines() {
            print!("{line}   ");
            let captures = regex.captures(line).unwrap();
            let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let c = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let d = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            if (a <= c && b >= d) || (c <= a && d >= b) {
                count += 1
            }
            println!("{:?}", (a <= c && b >= d) || (c <= a && d >= b))
        }
        count
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let regex = regex!(r"(\d+)-(\d+),(\d+)-(\d+)");
        let mut count = 0;
        for line in input.lines() {
            print!("{line}   ");
            let captures = regex.captures(line).unwrap();
            let a = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let c = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let d = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
            if (a < c && b < c) || (a > d && b > d) || (d < a && c < a) || (c > b && c > a) {
                count += 1
            }
            println!("{:?}", (a < c && b < c) || (a > d && b > d))
        }
        1000 - count
    }
}