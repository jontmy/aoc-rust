use std::str::FromStr;

use itertools::Itertools;
use num::Integer;
use scan_fmt::scan_fmt;

use crate::utils::{self, advent};

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: (String, String),
    test: usize,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.lines().collect_vec();
        let items = utils::get_all_nums::<usize>(s[1]).into_iter().collect();
        let op = scan_fmt!(s[2], "Operation: new = old {} {}", String, String).unwrap();
        let test = utils::get_all_nums::<usize>(s[3])[0];
        let if_true = utils::get_all_nums::<usize>(s[4])[0];
        let if_false = utils::get_all_nums::<usize>(s[5])[0];

        Ok(Self {
            items,
            op,
            test,
            if_true,
            if_false,
        })
    }
}

pub struct OldSolver;

impl OldSolver {
    fn monkeys(input: &str) -> Vec<Monkey> {
        input
            .trim()
            .split("\n\n")
            .filter_map(|m| m.parse::<Monkey>().ok())
            .collect_vec()
    }

    fn lcm(monkeys: &[Monkey]) -> usize {
        monkeys.iter().map(|m| m.test).fold(1, |acc, f| acc.lcm(&f))
    }

    fn round(
        monkeys: &mut [Monkey],
        counts: &mut [usize],
        worry_fn: impl Fn(usize) -> usize,
    ) -> Vec<Monkey> {
        let mut monkeys = monkeys.to_owned();
        for m_ptr in 0..monkeys.len() {
            let mut monkey = monkeys.get(m_ptr).unwrap().clone();
            monkeys[m_ptr].items.clear();
            for i_ptr in 0..monkey.items.len() {
                let mut worry = monkey.items[i_ptr];
                worry = match (monkey.op.0.as_str(), monkey.op.1.as_str()) {
                    ("*", "old") => worry * worry,
                    ("*", v) => worry * v.parse::<usize>().unwrap(),
                    ("+", v) => worry + v.parse::<usize>().unwrap(),
                    _ => panic!(),
                };
                worry = worry_fn(worry);
                counts[m_ptr] += 1;
                if worry % monkey.test == 0 {
                    monkeys[monkey.if_true].items.push(worry);
                } else {
                    monkeys[monkey.if_false].items.push(worry);
                }
            }
            monkey.items.clear();
        }
        monkeys
    }
}

impl advent::OldSolver<2022, 11> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut monkeys = OldSolver::monkeys(input);
        let mut counts = vec![0; monkeys.len()];
        for _ in 0..20 {
            monkeys = OldSolver::round(&mut monkeys, &mut counts, |worry| worry / 3);
        }
        counts.into_iter().sorted().rev().take(2).product()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut monkeys = OldSolver::monkeys(input);
        let mut counts = vec![0; monkeys.len()];
        let lcm = OldSolver::lcm(&monkeys);
        for _ in 0..10000 {
            monkeys = OldSolver::round(&mut monkeys, &mut counts, |worry| worry % lcm);
        }
        counts.into_iter().sorted().rev().take(2).product()
    }
}
