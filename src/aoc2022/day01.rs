use std::str::FromStr;

use itertools::Itertools;

use crate::utils::advent;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    food: i32,
}

impl FromStr for Elf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let food = s
            .split("\n")
            .into_iter()
            .map(|c| c.parse::<i32>().unwrap())
            .sum();
        Ok(Elf { food })
    }
}

pub struct Solver;

impl advent::Solver<2022, 1> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .trim()
            .split("\n\n")
            .map(|l| l.parse::<Elf>().unwrap())
            .max()
            .unwrap()
            .food
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let elves = input
            .trim()
            .split("\n\n")
            .map(|l| l.parse::<Elf>().unwrap());

        elves
            .sorted()
            .rev()
            .take(3)
            .map(|elf| elf.food)
            .sum::<i32>()
    }
}
