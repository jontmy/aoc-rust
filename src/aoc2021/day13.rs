use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;
use serde_scan::scan;

use crate::utils::coordinates::dim_2::Coordinates;

#[derive(Copy, Clone)]
struct Fold {
    axis: char,
    line: i32,
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, line) = scan!("fold along {}={}" <- s).unwrap();
        Ok(Fold { axis, line })
    }
}

struct Paper {
    dots: HashSet<Coordinates>,
    folds: Vec<Fold>,
}

impl FromStr for Paper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dots, folds) = s.split("\n\n").collect_tuple().unwrap();

        let dots = dots
            .lines()
            .map(|l| {
                let (a, b): (i32, i32) = scan!("{},{}" <- l).unwrap();
                Coordinates::from((a, b))
            })
            .collect::<HashSet<_>>();

        let folds = folds
            .lines()
            .map(Fold::from_str)
            .map(Result::unwrap)
            .collect_vec();

        Ok(Paper { dots, folds })
    }
}

impl Paper {
    fn dots_after_one_fold(self) -> HashSet<Coordinates> {
        Paper::fold(self.dots, self.folds[0])
    }

    fn dots_after_all_folds(self) -> HashSet<Coordinates> {
        self.folds
            .into_iter()
            .fold(self.dots, |dots, fold| Paper::fold(dots, fold))
    }

    fn fold(dots: HashSet<Coordinates>, fold: Fold) -> HashSet<Coordinates> {
        match fold.axis {
            'x' => dots
                .into_iter()
                .map(|dot| {
                    if dot.0 < fold.line {
                        dot
                    } else {
                        Coordinates::from((2 * fold.line - dot.0, dot.1))
                    }
                })
                .collect::<HashSet<_>>(),
            'y' => dots
                .into_iter()
                .map(|dot| {
                    if dot.1 < fold.line {
                        dot
                    } else {
                        Coordinates::from((dot.0, 2 * fold.line - dot.1))
                    }
                })
                .collect::<HashSet<_>>(),
            _ => panic!(),
        }
    }
}

pub fn solve_part_one(input: &String) -> usize {
    Paper::from_str(input).unwrap().dots_after_one_fold().len()
}

pub fn solve_part_two(input: &String) -> String {
    let dots = Paper::from_str(input).unwrap().dots_after_all_folds();

    let (x_min, x_max) = dots.iter().map(|c| c.0).minmax().into_option().unwrap();
    let (y_min, y_max) = dots.iter().map(|c| c.1).minmax().into_option().unwrap();

    let mut sb = String::from("\n");
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if dots.contains(&Coordinates::from((x, y))) {
                sb.push('#');
            } else {
                sb.push('.');
            }
        }
        sb.push('\n');
    }
    sb
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "}.to_string(), 17)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }
}
