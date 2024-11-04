use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::advent;

pub struct OldSolver;

impl OldSolver {
    const PRIORITIES: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn get_priority(c: char) -> usize {
        // O(n), but pretty self-explanatory and avoids ASCII manipulation.
        OldSolver::PRIORITIES.find(c).unwrap() + 1
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

impl advent::OldSolver<2022, 3> for OldSolver {
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
            .map(|intersection| OldSolver::get_priority(intersection))
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
            .map(|elves| OldSolver::intersect(elves))
            .map(|intersection| OldSolver::get_priority(intersection))
            .sum()
    }
}
