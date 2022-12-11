use std::{ str::FromStr, collections::VecDeque };

use itertools::Itertools;
use num::{BigUint, Zero, Integer};
use scan_fmt::scan_fmt;

use crate::utils::{ self, advent };

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: VecDeque<usize>,
    op: (String, String),
    test: usize,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.lines().collect_vec();
        let id = utils::get_all_nums::<usize>(s[0])[0];
        let items = utils::get_all_nums::<usize>(s[1]).into_iter().collect();
        let op = scan_fmt!(s[2], "Operation: new = old {} {}", String, String).unwrap();
        let test = utils::get_all_nums::<usize>(s[3])[0];
        let if_true = utils::get_all_nums::<usize>(s[4])[0];
        let if_false = utils::get_all_nums::<usize>(s[5])[0];

        Ok(Self { id, items, op, test, if_true, if_false })
    }
}

pub struct Solver;

impl advent::Solver<2022, 11> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut monkeys = input
            .trim()
            .split("\n\n")
            .filter_map(|m| m.parse::<Monkey>().ok())
            .collect_vec();

        let mut counts = vec![0; monkeys.len()];
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let mut monkeys_ref = monkeys.clone();
                let m_ptr = i;
                let monkey = monkeys_ref.get_mut(m_ptr).unwrap();
                monkeys[m_ptr].items.clear();
                while let Some(worry) = monkey.items.pop_front() {
                    let (a, b) = (monkey.op.0.as_str(), monkey.op.1.as_str());
                    let mut worry = match (a, b) {
                        ("+", v) => worry + v.parse::<usize>().unwrap(),
                        ("*", "old") => worry * worry,
                        ("*", v) => worry * v.parse::<usize>().unwrap(),
                        _ => panic!(),
                    };
                    worry = worry / 3;
                    counts[m_ptr] += 1;
                    if worry % monkey.test == 0 {
                        monkeys[monkey.if_true].items.push_back(worry);
                    } else {
                        monkeys[monkey.if_false].items.push_back(worry);
                    }
                }
            }
        }

        for monkey in monkeys.iter_mut() {
            println!("{:?}", monkey);
        }

        for count in &counts {
            println!("{count}");
        }

        counts.into_iter().sorted().rev().take(2).product()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut monkeys = input
            .trim()
            .split("\n\n")
            .filter_map(|m| m.parse::<Monkey>().ok())
            .collect_vec();
        let lcm = monkeys.iter()
            .map(|m| m.test)
            .fold(1, |acc, f| acc.lcm(&f));

        let mut counts = vec![0; monkeys.len()];
        for ii in 0..10000 {
            for i in 0..monkeys.len() {
                let mut monkeys_ref = monkeys.clone();
                let m_ptr = i;
                let monkey = monkeys_ref.get_mut(m_ptr).unwrap();
                monkeys[m_ptr].items.clear();
                while let Some(worry) = monkey.items.pop_front() {
                    let (a, b) = (monkey.op.0.as_str(), monkey.op.1.as_str());
                    let mut worry = match (a, b) {
                        ("+", v) => worry + v.parse::<usize>().unwrap(),
                        ("*", "old") => worry * worry,
                        ("*", v) => worry * v.parse::<usize>().unwrap(),
                        _ => panic!(),
                    };
                    worry %= lcm;
                    counts[m_ptr] += 1;
                    if worry % monkey.test == 0 {
                        monkeys[monkey.if_true].items.push_back(worry);
                    } else {
                        monkeys[monkey.if_false].items.push_back(worry);
                    }
                }
            }
            // dbg!(ii);
        }

        for monkey in monkeys.iter_mut() {
            println!("{:?}", monkey);
        }

        for count in &counts {
            println!("{count}");
        }

        counts.into_iter().sorted().rev().take(2).product()
    }
}