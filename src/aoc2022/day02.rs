use std::str::FromStr;

use itertools::Itertools;

use crate::utils::advent;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn evaluate(opponent: &Shape, player: &Shape) -> Self {
        match (opponent, player) {
            (Shape::Rock, Shape::Rock) => Self::Draw,
            (Shape::Rock, Shape::Paper) => Self::Win,
            (Shape::Rock, Shape::Scissors) => Self::Loss,
            (Shape::Paper, Shape::Rock) => Self::Loss,
            (Shape::Paper, Shape::Paper) => Self::Draw,
            (Shape::Paper, Shape::Scissors) => Self::Win,
            (Shape::Scissors, Shape::Rock) => Self::Win,
            (Shape::Scissors, Shape::Paper) => Self::Loss,
            (Shape::Scissors, Shape::Scissors) => Self::Draw,
        }
    }
}

pub struct Solver;

impl advent::Solver<2022, 2> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .lines()
            .filter_map(|l|
                l
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<Shape>().unwrap())
                    .collect_tuple::<(_, _)>()
            )
            .map(|(opponent, player)| Outcome::evaluate(&opponent, &player).score() + player.value())
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
        let mut score = 0;
        for line in input.lines() {
            let (opp, ply): (&str, &str) = line.split_ascii_whitespace().collect_tuple().unwrap();
            match opp {
                "A" =>
                    match ply {
                        "X" => {
                            score += 3;
                        }
                        "Y" => {
                            score += 1 + 3;
                        }
                        "Z" => {
                            score += 2 + 6;
                        }
                        _ => panic!(),
                    }
                "B" =>
                    match ply {
                        "X" => {
                            score += 1;
                        }
                        "Y" => {
                            score += 2 + 3;
                        }
                        "Z" => {
                            score += 3 + 6;
                        }
                        _ => panic!(),
                    }
                "C" =>
                    match ply {
                        "X" => {
                            score += 2;
                        }
                        "Y" => {
                            score += 3 + 3;
                        }
                        "Z" => {
                            score += 1 + 6;
                        }
                        _ => panic!(),
                    }
                _ => panic!(),
            }
        }

        score
    }
}