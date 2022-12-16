use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use regex::{Regex, Replacer};

pub fn solve_part_one(input: &String) -> usize {
    let (replacements, molecule) = parse_input(input);
    molecule.all_replacements(&replacements).len()
}

pub fn solve_part_two(input: &String) -> usize {
    let (_, medicine) = parse_input(input);
    let (caps, rn, ar, y) = ["[A-Z]", "Rn", "Ar", "Y"]
        .into_iter()
        .map(|condition| {
            Regex::new(condition)
                .unwrap()
                .captures_iter(medicine.formula.as_str())
                .count()
        })
        .collect_tuple()
        .unwrap();

    caps - rn - ar - 2 * y - 1

    /*
    // Brute force solution that will never return in a reasonable duration.

    let primordial = Molecule::from_str("e").unwrap();
    let (replacements, medicine) = parse_input(input);
    let mut queue = DoublePriorityQueue::new();
    queue.push((medicine.clone(), 0), medicine.clone().len());

    // Invert the replacements so we get from the medicine molecule to the primordial electron "e".
    let replacements = replacements.into_iter()
        .map(Replacement::invert)
        .collect_vec();

    let mut calculated = HashSet::new();
    loop {
        let ((molecule, steps), _) = queue.pop_min().unwrap();
        if molecule == primordial {
            return steps;
        }
        dbg!((&molecule.len(), &molecule));
        for molecule in molecule.all_replacements(&replacements).into_iter() {
            if calculated.contains(&molecule) {
                continue;
            }
            calculated.insert(molecule.clone());
            queue.push((molecule.clone(), steps + 1), molecule.clone().len());
        }
    }
    */
}

fn parse_input(input: &String) -> (Vec<Replacement>, Molecule) {
    let (replacements, molecule): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let replacements = replacements
        .lines()
        .map(|line| line.parse::<Replacement>().unwrap())
        .collect_vec();
    let molecule = molecule.parse::<Molecule>().unwrap();

    (replacements, molecule)
}

#[derive(Debug)]
struct Replacement {
    input: String,
    output: String,
}

impl FromStr for Replacement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s
            .trim()
            .split(" => ")
            .map(|s| s.to_string())
            .collect_tuple()
            .unwrap();

        Ok(Replacement { input, output })
    }
}

impl Replacement {
    fn invert(self) -> Self {
        Replacement {
            input: self.output,
            output: self.input,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Molecule {
    formula: String,
}

impl FromStr for Molecule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Molecule {
            formula: s.trim().to_string(),
        })
    }
}

impl Molecule {
    fn all_replacements(&self, replacements: &[Replacement]) -> Vec<Self> {
        replacements
            .into_iter()
            .flat_map(|replacement| {
                Regex::new(replacement.input.as_str())
                    .unwrap()
                    .find_iter(self.formula.as_str())
                    .map(|m| self.replace(replacement, m.start(), m.end()))
                    .collect_vec()
            })
            .unique()
            .collect_vec()
    }

    fn replace(&self, replacement: &Replacement, start: usize, end: usize) -> Self {
        let mut formula = self.formula.clone();
        formula.replace_range(start..end, replacement.output.as_str());

        Molecule { formula }
    }

    fn len(&self) -> usize {
        self.formula.len()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        H => HO
        H => OH
        O => HH

        HOH
    "}.to_string(), 4)]
    #[case(indoc::indoc ! {"
        H => HO
        H => OH
        O => HH

        HOHOHO
    "}.to_string(), 7)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOH
    "}.to_string(), 3)]
    #[case(indoc::indoc ! {"
        e => H
        e => O
        H => HO
        H => OH
        O => HH

        HOHOHO
    "}.to_string(), 6)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
