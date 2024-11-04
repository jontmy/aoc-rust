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

    fn from_outcome(opponent: &Shape, outcome: &Outcome) -> Self {
        match (opponent, outcome) {
            (Shape::Rock, Outcome::Win) => Self::Paper,
            (Shape::Rock, Outcome::Draw) => Self::Rock,
            (Shape::Rock, Outcome::Loss) => Self::Scissors,
            (Shape::Paper, Outcome::Win) => Self::Scissors,
            (Shape::Paper, Outcome::Draw) => Self::Paper,
            (Shape::Paper, Outcome::Loss) => Self::Rock,
            (Shape::Scissors, Outcome::Win) => Self::Rock,
            (Shape::Scissors, Outcome::Draw) => Self::Scissors,
            (Shape::Scissors, Outcome::Loss) => Self::Paper,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
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

pub struct OldSolver;

impl advent::OldSolver<2022, 2> for OldSolver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .lines()
            .filter_map(|l| {
                l.split_ascii_whitespace()
                    .map(|s| s.parse::<Shape>().unwrap())
                    .collect_tuple::<(_, _)>()
            })
            .map(|(opponent, player)| {
                Outcome::evaluate(&opponent, &player).score() + player.value()
            })
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        input
            .lines()
            .filter_map(|l| l.split_ascii_whitespace().collect_tuple::<(_, _)>())
            .map(|(a, b)| (a.parse::<Shape>().unwrap(), b.parse::<Outcome>().unwrap()))
            .map(|(opponent, outcome)| {
                Shape::from_outcome(&opponent, &outcome).value() + outcome.score()
            })
            .sum()
    }
}
