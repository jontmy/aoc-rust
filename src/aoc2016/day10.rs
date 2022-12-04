use std::{ str::FromStr, collections::HashMap };

use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::utils::advent;

#[derive(Debug)]
struct Instruction {
    bot: i32,
    lo: Target,
    hi: Target,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (bot, target_lo, lo, target_hi, hi) = scan_fmt!(
            s,
            "bot {d} gives low to {} {d} and high to {} {d}",
            i32,
            String,
            i32,
            String,
            i32
        ).unwrap();

        Ok(Instruction {
            bot,
            lo: Target::new(&target_lo, lo),
            hi: Target::new(&target_hi, hi),
        })
    }
}

#[derive(Debug)]
enum Target {
    Bot(i32),
    Output(i32),
}

impl Target {
    fn new(s: &str, i: i32) -> Self {
        match s {
            "bot" => Self::Bot(i),
            "output" => Self::Output(i),
            _ => panic!(),
        }
    }
}

pub struct Solver;

impl advent::Solver<2016, 10> for Solver {
    type Part1 = i32;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        // Parse the instructions, which when applied on all bots in a state with 2 microchips form a state transition.
        let instructions = input
            .lines()
            .filter(|l| l.starts_with("bot"))
            .map(|l| l.parse::<Instruction>().unwrap())
            .map(|i| (i.bot, i))
            .collect::<HashMap<_, _>>();

        // Initialize the state, at least one bot should have 2 microchips.
        let mut state = input
            .lines()
            .filter(|l| l.starts_with("value"))
            .filter_map(|l| scan_fmt!(l, "value {d} goes to bot {d}", i32, i32).ok())
            .map(|(value, bot)| (bot, value))
            .into_group_map();

        loop {
            let current = state.clone();
            // For every bot with 2 microchips, give it to the bot next in line based on its instruction.
            for (bot, microchips) in current.into_iter().filter(|(_, v)| v.len() == 2) {
                let (lo, hi) = (microchips[0].min(microchips[1]), microchips[0].max(microchips[1]));
                let instruction = &instructions[&bot];
                state.remove(&bot);
                // Ignore output bins.
                if let Target::Bot(target_lo) = instruction.lo {
                    state.entry(target_lo).or_insert(Vec::new()).push(lo);
                }
                if let Target::Bot(target_hi) = instruction.hi {
                    state.entry(target_hi).or_insert(Vec::new()).push(hi);
                }
                if lo == 17 && hi == 61 {
                    return bot;
                }
            }
        }
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        0
    }
}