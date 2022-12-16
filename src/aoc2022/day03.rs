use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::advent;

pub struct Solver;

impl Solver {
    const PRIORITIES: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn get_priority(c: char) -> usize {
        // O(n), but pretty self-explanatory and avoids ASCII manipulation.
        Solver::PRIORITIES.find(c).unwrap() + 1
    }

    fn intersect(sets: (HashSet<char>, HashSet<char>, HashSet<char>)) -> char {
        let (a, b, c) = sets;
        a.intersection(&b)
            .cloned()
            .collect::<HashSet<_>>()
            .intersection(&c)
            .next()
            .unwrap()
            .clone()
    }
}

impl advent::Solver<2022, 3> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .trim()
            .lines()
            .map(|l| {
                (
                    l[0..l.len() / 2].chars().collect::<HashSet<_>>(),
                    l[l.len() / 2..].chars().collect::<HashSet<_>>(),
                )
            })
            .map(|(a, b)| a.intersection(&b).next().unwrap().clone())
            .map(|intersection| Solver::get_priority(intersection))
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let groups = input.trim().lines().chunks(3);
        groups
            .into_iter()
            .map(|group| {
                group
                    .map(|elf| elf.chars().collect::<HashSet<_>>())
                    .collect_tuple::<(_, _, _)>()
                    .unwrap()
            })
            .map(|elves| Solver::intersect(elves))
            .map(|intersection| Solver::get_priority(intersection))
            .sum()
    }
}
