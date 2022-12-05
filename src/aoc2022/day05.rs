use std::{collections::VecDeque, vec};

use crate::utils::advent;

use itertools::Itertools;
use scan_fmt::scan_fmt;

pub struct Solver;

impl advent::Solver<2022, 5> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut stacks = vec![
            VecDeque::from_iter("MSJLVFNR".chars()),
            VecDeque::from_iter("HWJFZDNP".chars()),
            VecDeque::from_iter("GDCRW".chars()),
            VecDeque::from_iter("SBN".chars()),
            VecDeque::from_iter("NFBCPWZM".chars()),
            VecDeque::from_iter("WMRP".chars()),
            VecDeque::from_iter("WSLGNTR".chars()),
            VecDeque::from_iter("VBNFHTQ".chars()),
            VecDeque::from_iter("FNZHML".chars()),
        ];
        let (_, ins) = input.split("\n\n").collect_tuple::<(_, _)>().unwrap();
        println!("{ins}");
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
        let mut stacks = vec![
            VecDeque::from_iter("MSJLVFNR".chars()),
            VecDeque::from_iter("HWJFZDNP".chars()),
            VecDeque::from_iter("GDCRW".chars()),
            VecDeque::from_iter("SBN".chars()),
            VecDeque::from_iter("NFBCPWZM".chars()),
            VecDeque::from_iter("WMRP".chars()),
            VecDeque::from_iter("WSLGNTR".chars()),
            VecDeque::from_iter("VBNFHTQ".chars()),
            VecDeque::from_iter("FNZHML".chars()),
        ];
        let (_, ins) = input.split("\n\n").collect_tuple::<(_, _)>().unwrap();
        println!("{ins}");
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