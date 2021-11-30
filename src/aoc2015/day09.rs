use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    lazy_static!(
        static ref RE: Regex = Regex::new(r"(?m)^(\w+) to (\w+) = (\d+)$").unwrap();
    );

    // Construct the undirected graph of distances between any two valid locations:
    // HashMap<<location1: &str, location2: &str>, distance: i32>.
    let distances = input.lines()
        .map(|line| RE.captures(line).unwrap())
        .map(|capture| (capture.get(1).unwrap().as_str(),
                        capture.get(2).unwrap().as_str(),
                        capture.get(3).unwrap().as_str().parse().unwrap()))
        .flat_map(|(l1, l2, d)| [((l1, l2), d), ((l2, l1), d)]) // the graph is undirected, so add edges in both directions
        .collect::<HashMap<(&str, &str), i32>>();

    // Find all unique locations.
    let locations = distances.keys()
        .map(|(l, _)| *l)
        .collect::<HashSet<&str>>();

    // Construct all routes and accept the shortest one.
    locations.iter()
        .permutations(locations.len())
        .map(|route| { // map each route permutation to the total distance for the route
            (0..route.len() - 1).into_iter()
                .map(|i| (*route[i], *route[i + 1]))
                .filter_map(|edge| distances.get(&edge)) // discard invalid routes
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

pub fn solve_part_two(input: &String) -> i32 {
    lazy_static!(
        static ref RE: Regex = Regex::new(r"(?m)^(\w+) to (\w+) = (\d+)$").unwrap();
    );

    // Construct the undirected graph of distances between any two valid locations:
    // HashMap<<location1: &str, location2: &str>, distance: i32>.
    let distances = input.lines()
        .map(|line| RE.captures(line).unwrap())
        .map(|capture| (capture.get(1).unwrap().as_str(),
                        capture.get(2).unwrap().as_str(),
                        capture.get(3).unwrap().as_str().parse().unwrap()))
        .flat_map(|(l1, l2, d)| [((l1, l2), d), ((l2, l1), d)]) // the graph is undirected, so add edges in both directions
        .collect::<HashMap<(&str, &str), i32>>();

    // Find all unique locations.
    let locations = distances.keys()
        .map(|(l, _)| *l)
        .collect::<HashSet<&str>>();

    // Construct all routes and accept the longest one.
    locations.iter()
        .permutations(locations.len())
        .map(|route| { // map each route permutation to the total distance for the route
            (0..route.len() - 1).into_iter()
                .map(|i| (*route[i], *route[i + 1]))
                .filter_map(|edge| distances.get(&edge)) // discard invalid routes
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "}.to_string(), 605)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "}.to_string(), 982)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}