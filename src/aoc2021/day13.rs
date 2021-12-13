use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use crate::utils::coordinates::dim_2::Coordinates;
use itertools::Itertools;
use serde_scan::scan;

fn parse_input(input: &String) -> (HashSet<Coordinates>, Vec<(char, i32)>) {
    let (dots, folds): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let dots = dots
        .lines()
        .map(|l| {
            let (a, b): (i32, i32) = scan!("{},{}" <- l).unwrap();
            Coordinates::from((a, b))
        })
        .collect::<HashSet<_>>();

    let folds = folds
        .lines()
        .map(|l| scan!("fold along {}={}" <- l).unwrap())
        .collect::<Vec<(char, i32)>>();

    (dots, folds)
}

pub fn solve_part_one(input: &String) -> usize {
    let (mut dots, folds) = parse_input(input);
    for (axis, fold) in folds.into_iter().take(1) {
        match axis {
            'x' => {
                let (untouched, mut folded): (Vec<Coordinates>, Vec<Coordinates>) =
                    dots.iter().partition(|c| c.0 < fold);
                for dot in folded.iter_mut() {
                    dot.0 = fold - (dot.0 - fold).abs();
                }
                dots = untouched
                    .into_iter()
                    .chain(folded.into_iter())
                    .collect::<HashSet<Coordinates>>();
            }
            'y' => {
                let (untouched, mut folded): (Vec<Coordinates>, Vec<Coordinates>) =
                    dots.iter().partition(|c| c.1 < fold);
                for dot in folded.iter_mut() {
                    dot.1 = fold - (dot.1 - fold).abs();
                }
                dots = untouched
                    .into_iter()
                    .chain(folded.into_iter())
                    .collect::<HashSet<Coordinates>>();
            }
            _ => panic!(),
        }
    }

    dots.len()
}

pub fn solve_part_two(input: &String) -> i32 {
    let (mut dots, folds) = parse_input(input);

    for (axis, fold) in folds {
        match axis {
            'x' => {
                let (untouched, mut folded): (Vec<Coordinates>, Vec<Coordinates>) =
                    dots.iter().partition(|c| c.0 < fold);
                for dot in folded.iter_mut() {
                    dot.0 = (fold) - (dot.0 - fold).abs();
                }
                dots = untouched
                    .into_iter()
                    .chain(folded.into_iter())
                    .collect::<HashSet<Coordinates>>();
            }
            'y' => {
                let (untouched, mut folded): (Vec<Coordinates>, Vec<Coordinates>) =
                    dots.iter().partition(|c| c.1 < fold);
                for dot in folded.iter_mut() {
                    dot.1 = (fold) - (dot.1 - fold).abs();
                }
                dots = untouched
                    .into_iter()
                    .chain(folded.into_iter())
                    .collect::<HashSet<Coordinates>>();
            }
            _ => panic!(),
        }
    }

    for y in 0..10 {
        for x in 0..20 {
            if dots.contains(&Coordinates::from((x, y))) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }

    dots.len() as i32
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

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
