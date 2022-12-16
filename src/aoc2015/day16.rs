use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)")
        .unwrap()
        .captures_iter(input)
        .into_iter()
        .map(|capture| {
            let number = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let compounds = (0..3)
                .into_iter()
                .map(|i| {
                    (
                        capture.get(i * 2 + 2).unwrap().as_str(),
                        capture
                            .get(i * 2 + 3)
                            .unwrap()
                            .as_str()
                            .parse::<i32>()
                            .unwrap(),
                    )
                })
                .collect::<HashMap<&str, i32>>();
            (number, compounds)
        })
        .filter(|(_, compounds)| {
            compounds.get("children").map(|i| *i == 3).unwrap_or(true)
                && compounds.get("cats").map(|i| *i == 7).unwrap_or(true)
                && compounds.get("samoyeds").map(|i| *i == 2).unwrap_or(true)
                && compounds
                    .get("pomeranians")
                    .map(|i| *i == 3)
                    .unwrap_or(true)
                && compounds.get("akitas").map(|i| *i == 0).unwrap_or(true)
                && compounds.get("vizslas").map(|i| *i == 0).unwrap_or(true)
                && compounds.get("goldfish").map(|i| *i == 5).unwrap_or(true)
                && compounds.get("trees").map(|i| *i == 3).unwrap_or(true)
                && compounds.get("cars").map(|i| *i == 2).unwrap_or(true)
                && compounds.get("perfumes").map(|i| *i == 1).unwrap_or(true)
        })
        .map(|(number, _)| number)
        .next()
        .unwrap()
}

pub fn solve_part_two(input: &String) -> i32 {
    Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)")
        .unwrap()
        .captures_iter(input)
        .into_iter()
        .map(|capture| {
            let number = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let compounds = (0..3)
                .into_iter()
                .map(|i| {
                    (
                        capture.get(i * 2 + 2).unwrap().as_str(),
                        capture
                            .get(i * 2 + 3)
                            .unwrap()
                            .as_str()
                            .parse::<i32>()
                            .unwrap(),
                    )
                })
                .collect::<HashMap<&str, i32>>();
            (number, compounds)
        })
        .filter(|(_, compounds)| {
            compounds.get("children").map(|i| *i == 3).unwrap_or(true)
                && compounds.get("cats").map(|i| *i > 7).unwrap_or(true)
                && compounds.get("samoyeds").map(|i| *i == 2).unwrap_or(true)
                && compounds.get("pomeranians").map(|i| *i < 3).unwrap_or(true)
                && compounds.get("akitas").map(|i| *i == 0).unwrap_or(true)
                && compounds.get("vizslas").map(|i| *i == 0).unwrap_or(true)
                && compounds.get("goldfish").map(|i| *i < 5).unwrap_or(true)
                && compounds.get("trees").map(|i| *i > 3).unwrap_or(true)
                && compounds.get("cars").map(|i| *i == 2).unwrap_or(true)
                && compounds.get("perfumes").map(|i| *i == 1).unwrap_or(true)
        })
        .map(|(number, _)| number)
        .next()
        .unwrap()
}
