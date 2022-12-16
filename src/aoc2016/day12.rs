use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::utils::advent;

type Register = char;

#[derive(Debug)]
enum Instruction {
    Set(i32, Register),
    Copy(Register, Register),
    Increment(Register),
    Decrement(Register),
    Jump(Register, i32),
    Skip(i32),
    Nop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect_vec();
        let rs = parts[1].chars().next().unwrap();
        let op = match parts[0] {
            "cpy" => {
                let rd = parts[2].chars().next().unwrap();
                match parts[1].parse::<i32>() {
                    Ok(imm) => Self::Set(imm, rd),
                    Err(_) => Self::Copy(rs, rd),
                }
            }
            "inc" => Self::Increment(rs),
            "dec" => Self::Decrement(rs),
            "jnz" => {
                let offset = parts[2].parse::<i32>().unwrap();
                match parts[1].parse::<i32>() {
                    Ok(0) => Self::Nop,
                    Ok(_) => Self::Skip(offset),
                    Err(_) => Self::Jump(rs, offset),
                }
            }
            _ => panic!(),
        };
        Ok(op)
    }
}

pub struct Solver;

impl Solver {
    fn exec(instructions: Vec<Instruction>, mut registers: HashMap<char, i32>) -> i32 {
        let mut pc = 0;
        while pc < (instructions.len() as i32) {
            match instructions[pc as usize] {
                Instruction::Set(imm, reg) => {
                    registers.insert(reg, imm);
                }
                Instruction::Copy(rs, rd) => {
                    registers.insert(rd, registers[&rs]);
                }
                Instruction::Increment(rd) => {
                    registers.entry(rd).and_modify(|v| {
                        *v += 1;
                    });
                }
                Instruction::Decrement(rd) => {
                    registers.entry(rd).and_modify(|v| {
                        *v -= 1;
                    });
                }
                Instruction::Jump(rs, offset) => match registers[&rs] {
                    0 => (),
                    _ => {
                        pc += offset;
                        continue;
                    }
                },
                Instruction::Skip(offset) => {
                    pc += offset;
                    continue;
                }
                Instruction::Nop => (),
            }
            pc += 1;
        }
        registers[&'a']
    }
}

impl advent::Solver<2016, 12> for Solver {
    type Part1 = i32;
    type Part2 = i32;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let instructions = input
            .lines()
            .filter_map(|l| l.parse::<Instruction>().ok())
            .collect_vec();

        let mut registers = HashMap::new();
        for reg in "abcd".chars() {
            registers.insert(reg, 0);
        }
        Solver::exec(instructions, registers)
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let instructions = input
            .lines()
            .filter_map(|l| l.parse::<Instruction>().ok())
            .collect_vec();

        let mut registers = HashMap::new();
        for reg in "abd".chars() {
            registers.insert(reg, 0);
        }
        registers.insert('c', 1);
        Solver::exec(instructions, registers)
    }
}
