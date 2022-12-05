use std::{ collections::VecDeque, vec };

use crate::utils::advent;

use itertools::Itertools;
use scan_fmt::scan_fmt;

pub struct Solver;

impl Solver {
    fn parse(input: &str) -> [VecDeque<char>; 9] {
        let crates = input
            .lines()
            .take_while(|l| l.starts_with("["))
            .flat_map(|l|
                l
                    .chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| c.is_alphabetic())
            )
            .into_grouping_map()
            .collect::<VecDeque<char>>();

        let crates: [VecDeque<char>; 9] = crates
            .into_iter()
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, stack)| stack)
            .collect_vec()
            .try_into()
            .unwrap();

        crates
    }
}

impl advent::Solver<2022, 5> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut stacks = Solver::parse(input);
        let (_, ins) = input.split("\n\n").collect_tuple::<(_, _)>().unwrap();
        let instructions = ins
            .lines()
            .filter_map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).ok())
            .collect_vec();
        for (amt, from, to) in instructions {
            for _ in 0..amt {
                let item = stacks[from - 1].pop_front().unwrap();
                stacks[to - 1].push_front(item);
            }
        }
        for i in 0..9 {
            print!("{:?}", stacks[i].pop_front().unwrap());
        }
        0
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut stacks = Solver::parse(input);
        let (_, ins) = input.split("\n\n").collect_tuple::<(_, _)>().unwrap();
        let instructions = ins
            .lines()
            .filter_map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).ok())
            .collect_vec();
        let mut holding = VecDeque::new();
        for (amt, from, to) in instructions {
            for _ in 0..amt {
                let item = stacks[from - 1].pop_front().unwrap();
                holding.push_back(item);
            }
            for _ in 0..amt {
                let item = holding.pop_back().unwrap();
                stacks[to - 1].push_front(item);
            }
        }
        for i in 0..9 {
            print!("{:?}", stacks[i].pop_front().unwrap());
        }
        0
    }
}