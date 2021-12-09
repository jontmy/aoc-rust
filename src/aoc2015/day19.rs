use std::str::FromStr;
use itertools::Itertools;
use regex::{Regex, Replacer};

pub fn solve_part_one(input: &String) -> usize {
    let (replacements, molecule) = parse_input(input);

    replacements.into_iter()
        .flat_map(|replacement| molecule.all_replacements(&replacement))
        .unique()
        .count()
}

pub fn solve_part_two(input: &String) -> usize {
    0
}

fn parse_input(input: &String) ->(Vec<Replacement>, Molecule) {
    let (replacements, molecule): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let replacements = replacements.lines()
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
        let (input, output) = s.trim().split(" => ")
            .map(|s| s.to_string())
            .collect_tuple().unwrap();

        Ok(Replacement {
            input, output
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Molecule {
    formula: String,
}

impl FromStr for Molecule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Molecule{
            formula: s.trim().to_string(),
        })
    }
}

impl Molecule {
    fn all_replacements(&self, replacement: &Replacement) -> Vec<Self> {
        Regex::new(replacement.input.as_str()).unwrap()
            .find_iter(self.formula.as_str())
            .map(|m| self.replace(replacement, m.start(), m.end()))
            .collect_vec()
    }

    fn replace(&self, replacement: &Replacement, start: usize, end: usize) -> Self {
        let mut formula = self.formula.clone();
        formula.replace_range(start..end, replacement.output.as_str());

        Molecule {
            formula
        }
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