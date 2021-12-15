use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::ops::Deref;
use std::str::FromStr;

use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;
use petgraph::graphmap::DiGraphMap;
use petgraph::{Directed, Graph};
use regex::Regex;
use serde_scan::scan;

use crate::utils::coordinates::dim_2::Coordinates;

pub fn solve_part_one(input: &String) -> u32 {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().chars().count() as i32;

    let risks = input.lines()
        .flat_map(&str::chars)
        .map(|risk| risk.to_digit(10).unwrap());

    let risks = Coordinates::in_area(0..width, 0..height)
        .zip(risks)
        .collect::<HashMap<Coordinates, u32>>();

    let graph = risks.keys()
        .flat_map(|c| {
            c.axial_offset_by(1)
                .filter(|d| d.x() >= 0 && d.y() >= 0 && d.x() < width && d.y() < height)
                .map(|d| (c.clone(), d, risks.get(&d).unwrap()))
        })
        .collect::<DiGraphMap<Coordinates, u32>>();

    *dijkstra(
        &graph,
        Coordinates::at(0, 0),
        Some(Coordinates::at(width - 1, height - 1)),
        |(.., w)| *w,
    )
    .get(&Coordinates::at(width - 1, height - 1)).unwrap()
}

pub fn solve_part_two(input: &String) -> u32 {
    let mut sb = String::new();
    for i in 0..5 {
        for line in input.lines() {
            for j in 0..5 {
                for risk in line.chars() {
                    let risk = risk.to_digit(10).unwrap() + i + j;
                    let risk = if risk > 9 { risk - 9 } else { risk };
                    sb.push(char::from_digit(risk, 10).unwrap());
                }
            }
            sb.push('\n');
        }
    }
    solve_part_one(&sb)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "}.to_string(), 40)]
    fn test_part_one(#[case] input: String, #[case] expected: u32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "}.to_string(), 315)]
    fn test_part_two(#[case] input: String, #[case] expected: u32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
