use std::str::FromStr;

use itertools::Itertools;

use crate::utils::advent;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl advent::Solver<2022, 2> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut score = 0;
        for line in input.lines() {
            let (opp, ply): (&str, &str) = line.split_ascii_whitespace().collect_tuple().unwrap();
            match opp {
                "A" => match ply {
                    "X" => score += 1 + 3,
                    "Y" => score += 2 + 6,
                    "Z" => score += 3,
                    _ => panic!()
                },
                "B" => match ply {
                    "X" => score += 1,
                    "Y" => score += 2 + 3,
                    "Z" => score += 3 + 6,
                    _ => panic!()
                },
                "C" => match ply {
                    "X" => score += 1 + 6,
                    "Y" => score += 2,
                    "Z" => score += 3 + 3,
                    _ => panic!()
                },
                _ => panic!()
            }
        }

        score
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
        let mut score = 0;
        for line in input.lines() {
            let (opp, ply): (&str, &str) = line.split_ascii_whitespace().collect_tuple().unwrap();
            match opp {
                "A" => match ply {
                    "X" => score += 3,
                    "Y" => score += 1 + 3,
                    "Z" => score += 2 + 6,
                    _ => panic!()
                },
                "B" => match ply {
                    "X" => score += 1,
                    "Y" => score += 2 + 3,
                    "Z" => score += 3 + 6,
                    _ => panic!()
                },
                "C" => match ply {
                    "X" => score += 2,
                    "Y" => score += 3 + 3,
                    "Z" => score += 1 + 6,
                    _ => panic!()
                },
                _ => panic!()
            }
        }

        score
    }
}