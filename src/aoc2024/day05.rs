use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::v2::{parser::get_all_ints_signed, solver};

type Update = Vec<i64>;
type Updates = Vec<Update>;
type Constraints = HashMap<i64, Vec<i64>>;
type Input = (Constraints, Updates);

impl Solver {
    fn parse_input(&self, input: &str) -> Input {
        let (constraints, updates) = input
            .split("\n\n")
            .map(|section| section.lines().map(get_all_ints_signed).collect_vec())
            .collect_tuple()
            .unwrap();

        let constraints = constraints
            .into_iter()
            .map(|v| (v[0], v[1]))
            .into_group_map();

        assert!(updates.iter().all(|v| v.len() % 2 == 1));
        (constraints, updates)
    }

    fn satisfies_constraints(&self, constraints: &Constraints, update: &Update) -> bool {
        for (i, r) in update.iter().enumerate() {
            if constraints.get(r).is_some() {
                for l in &update[..i] {
                    if constraints.get(&r).unwrap().contains(l) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

pub struct Solver;

impl solver::Solver<2024, 5> for Solver {
    type Part1 = i64;
    type Part2 = i64;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let (constraints, updates) = self.parse_input(input);

        updates
            .into_iter()
            .filter(|update| self.satisfies_constraints(&constraints, update))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let (constraints, updates) = self.parse_input(input);
        let mut result = 0;
        for mut update in updates {
            if self.satisfies_constraints(&constraints, &update) {
                continue;
            }
            while !self.satisfies_constraints(&constraints, &update) {
                for i in 0..update.len() {
                    for j in i..update.len() {
                        if let Some(c) = constraints.get(&update[j]) {
                            if c.contains(&update[i]) {
                                update.swap(i, j);
                            }
                        }
                    }
                }
            }
            result += update[update.len() / 2];
        }
        result
    }
}
