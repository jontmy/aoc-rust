use std::{ str::FromStr, collections::HashSet, ops::Index, fmt::format };

use itertools::Itertools;

use crate::utils::advent;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
impl Solver {
    const PRIORITIES: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    fn get_priority(c: char) -> usize {
        Solver::PRIORITIES.find(c).unwrap() + 1
    }
}

impl advent::Solver<2022, 3> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .trim()
            .lines()
            .map(|l| (
                l[0..l.len() / 2].chars().collect::<HashSet<char>>(),
                l[l.len() / 2..].chars().collect::<HashSet<char>>(),
            ))
            .map(|(a, b)| a.intersection(&b).next().unwrap().clone())
            .map(|intersection| Solver::get_priority(intersection))
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let pairs = input
            .trim()
            .lines()
            .tuples::<(_, _, _)>()
            .collect_vec();
        let s = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut sum = 0;
        for (a, b, c) in pairs {
            let a = a.chars().collect::<HashSet<char>>();
            let b = b.chars().collect::<HashSet<char>>();
            let c = c.chars().collect::<HashSet<char>>();
            let inter = a.intersection(&b).map(|c| *c).collect::<HashSet<char>>();
            let inter = inter.intersection(&c).collect_vec();
            let d = inter[0].to_string();
            println!("{}", s.find(&d).unwrap() + 1);
            sum += s.find(&d).unwrap() + 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {}