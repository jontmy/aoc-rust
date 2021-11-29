use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use lazy_static::lazy_static;
use regex::Regex;

struct Operation<'a> {
    // optional input
    io: Option<&'a str>,
    // operation or assignment (if omitted)
    op: &'a str,
    // compulsory input
    ic: &'a str,
    // compulsory output
    o: &'a str,
}

impl Operation<'_> {
    fn parse(s: &str) -> Operation {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w*)*? ?(AND|OR|NOT|LSHIFT|RSHIFT)? ?(\w*) ? -> ([a-z]{1,2})$").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        Operation {
            io: captures.get(1).map(|m| m.as_str()),
            op: captures.get(2).map_or("NOP", |m| m.as_str()),
            ic: captures.get(3).map(|m| m.as_str()).unwrap(),
            o: captures.get(4).map(|m| m.as_str()).unwrap(),
        }
    }
}

impl Debug for Operation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f,
                "{} {} {} -> {}",
                self.io.unwrap_or("_"),
                self.op,
                self.ic,
                self.o
        }
    }
}

struct Machine {
    wires: HashMap<String, u16>,
}

impl Machine {
    fn new() -> Machine {
        Machine { wires: HashMap::new() }
    }

    fn perform_operation(&mut self, op: &Operation) {
        let result: u16 = match op.op {
            "NOP" => self.get_wire_value(op.ic),
            "AND" => self.get_wire_value(op.io.unwrap()) & self.get_wire_value(op.ic),
            "OR" => self.get_wire_value(op.io.unwrap()) | self.get_wire_value(op.ic),
            "NOT" => !self.get_wire_value(op.ic),
            "LSHIFT" => self.get_wire_value(op.io.unwrap()) << self.get_wire_value(op.ic),
            "RSHIFT" => self.get_wire_value(op.io.unwrap()) >> self.get_wire_value(op.ic),
            _ => panic!()
        };
        self.wires.insert(String::from(op.o), result);
    }

    fn get_wire_value(&self, wire: &str) -> u16 {
        if self.wires.contains_key(wire) {
            *self.wires.get(wire).unwrap()
        } else {
            wire.parse::<u16>().unwrap_or(0)
        }
    }
}

pub fn solve_part_one(input: &String) -> u16 {
    let mut machine = Machine::new();
    let operations = input.lines()
        .map(Operation::parse)
        .fold(HashMap::new(), |mut acc, op| {
            acc.insert(op.o, op);
            acc
        });

    // Wires are connected in reverse alphabetical order.
    let mut wires = operations.keys()
        .map(|w| *w)
        .collect::<Vec<&str>>();
    wires.sort();

    // Wires with one letter are connected before those with two letters.
    let singular = wires.iter()
        .filter(|w| w.len() == 1)
        .map(|w| operations.get(w).unwrap());

    let plural = wires.iter()
        .filter(|w| w.len() == 2)
        .map(|w| operations.get(w).unwrap());

    singular.chain(plural)
        .skip(1) // skip the connection of wire 'a' till the end
        .for_each(|op| machine.perform_operation(op));

    machine.perform_operation(operations.get("a").unwrap());
    machine.get_wire_value("a")
}

pub fn solve_part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::solve_part_one;

    #[rstest]
    #[case(indoc::indoc ! {"
        123 -> a
    "}.to_string(), 123)]
    #[case(indoc::indoc ! {"
        123 -> x
        456 -> y
        x AND y -> a
    "}.to_string(), 72)]
    #[case(indoc::indoc ! {"
        123 -> x
        456 -> y
        x OR y -> a
    "}.to_string(), 507)]
    #[case(indoc::indoc ! {"
        123 -> x
        x LSHIFT 2 -> a
    "}.to_string(), 492)]
    #[case(indoc::indoc ! {"
        456 -> y
        y RSHIFT 2 -> a
    "}.to_string(), 114)]
    #[case(indoc::indoc ! {"
        123 -> x
        NOT x -> a
    "}.to_string(), 65412)]
    #[case(indoc::indoc ! {"
        456 -> y
        NOT y -> a
    "}.to_string(), 65079)]
    fn test_part_one(#[case] input: String, #[case] expected: u16) {
        assert_eq!(expected, solve_part_one(&input))
    }
}