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
            i32, String, i32, String, i32
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

impl Solver {
    fn get_instructions(input: &str) -> HashMap<i32, Instruction> {
        input
            .lines()
            .filter(|l| l.starts_with("bot"))
            .map(|l| l.parse::<Instruction>().unwrap())
            .map(|i| (i.bot, i))
            .collect::<HashMap<_, _>>()
    }

    fn get_initial_state(input: &str) -> HashMap<i32, Vec<i32>> {
        input
            .lines()
            .filter(|l| l.starts_with("value"))
            .filter_map(|l| scan_fmt!(l, "value {d} goes to bot {d}", i32, i32).ok())
            .map(|(value, bot)| (bot, value))
            .into_group_map()
    }
}

impl advent::Solver<2016, 10> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let instructions = Solver::get_instructions(input);
        let mut state = Solver::get_initial_state(input);
        loop {
            let current = state.clone();
            // For every bot with 2 microchips, give it to the bot next in line based on its instruction.
            for (bot, microchips) in current.into_iter().filter(|(_, v)| v.len() == 2) {
                let (lo, hi) = (microchips[0].min(microchips[1]), microchips[0].max(microchips[1]));
                let instruction = &instructions[&bot];
                state.remove(&bot);
                // Ignore output bins.
                if let Target::Bot(bot) = instruction.lo {
                    state.entry(bot).or_insert(Vec::new()).push(lo);
                }
                if let Target::Bot(bot) = instruction.hi {
                    state.entry(bot).or_insert(Vec::new()).push(hi);
                }
                if lo == 17 && hi == 61 {
                    return bot;
                }
            }
        }
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let instructions = Solver::get_instructions(input);
        let mut state = Solver::get_initial_state(input);
        let mut bins = HashMap::new();
        while !bins.contains_key(&0) || !bins.contains_key(&1) || !bins.contains_key(&2) {
            let current = state.clone();
            // For every bot with 2 microchips, give it to the bot next in line based on its instruction.
            for (bot, microchips) in current.into_iter().filter(|(_, v)| v.len() == 2) {
                let (lo, hi) = (microchips[0].min(microchips[1]), microchips[0].max(microchips[1]));
                let instruction = &instructions[&bot];
                state.remove(&bot);
                match instruction.lo {
                    Target::Bot(bot) => state.entry(bot).or_insert(Vec::new()).push(lo),
                    Target::Output(bin) => {
                        bins.insert(bin, lo);
                    }
                }
                match instruction.hi {
                    Target::Bot(bot) => state.entry(bot).or_insert(Vec::new()).push(hi),
                    Target::Output(bin) => {
                        bins.insert(bin, hi);
                    }
                }
            }
        }
        bins[&0] * bins[&1] * bins[&2]
    }
}