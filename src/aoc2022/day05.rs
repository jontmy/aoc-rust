use std::collections::VecDeque;

use crate::utils::aoc;

use itertools::Itertools;
use scan_fmt::scan_fmt;

struct Instruction {
    qty: usize,
    from: usize,
    to: usize,
}

pub struct OldSolver;

impl OldSolver {
    fn parse(input: &str) -> (Vec<VecDeque<char>>, impl Iterator<Item = Instruction> + '_) {
        let (crates, instructions) = input.split("\n\n").into_iter().collect_tuple().unwrap();

        let crates = crates
            .lines()
            .flat_map(|l| {
                l.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| c.is_alphabetic())
            })
            .into_grouping_map()
            .collect::<VecDeque<char>>();

        let crates = crates
            .into_iter()
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, stack)| stack)
            .collect();

        let instructions = instructions
            .lines()
            .filter_map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).ok())
            .map(|(qty, from, to)| Instruction {
                qty,
                from: from - 1,
                to: to - 1,
            });

        (crates, instructions)
    }
}

impl aoc::OldSolver<2022, 5> for OldSolver {
    type Part1 = String;
    type Part2 = String;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let (mut stacks, instructions) = OldSolver::parse(input);
        for ins in instructions {
            for _ in 0..ins.qty {
                let item = stacks[ins.from].pop_front().unwrap();
                stacks[ins.to].push_front(item);
            }
        }
        stacks
            .into_iter()
            .filter_map(|mut stack| stack.pop_front())
            .collect()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let (mut stacks, instructions) = OldSolver::parse(input);
        let mut holding = VecDeque::new();
        for ins in instructions {
            for _ in 0..ins.qty {
                let item = stacks[ins.from].pop_front().unwrap();
                holding.push_back(item);
            }
            for _ in 0..ins.qty {
                let item = holding.pop_back().unwrap();
                stacks[ins.to].push_front(item);
            }
        }
        stacks
            .into_iter()
            .filter_map(|mut stack| stack.pop_front())
            .collect()
    }
}
