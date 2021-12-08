use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

// REFACTORING IN PROGRESS

pub fn solve_part_one(input: &String) -> i32 {
    input.lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .map(|entry| entry.count1478())
        .sum()
}

pub fn solve_part_two(input: &String) -> i32 {
    let entries = input.lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .collect_vec();

    let mut sum = 0;
    for entry in entries {
        let deduced = entry.deduce();
        let output = entry.digits;
        let mut actual = String::new();

        for digit in output {
            let n = deduced.iter()
                .position(|ded| digit.len() == ded.len() && digit.intersection(ded).count() == digit.len())
                .unwrap();

            actual.push_str(&*n.to_string());
        }
        let parsed = actual.parse::<i32>().unwrap();
        sum += parsed;
    }
    sum
}

#[derive(Debug)]
struct Entry {
    signals: Vec<HashSet<char>>,
    digits: Vec<HashSet<char>>,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let signals = s.trim()
            .split('|')
            .flat_map(|substr| substr.split_whitespace())
            .map(|signal| signal.chars().collect::<HashSet<_>>())
            .take(10)
            .collect_vec();

        let digits = s.trim()
            .split('|')
            .flat_map(|substr| substr.split_whitespace())
            .map(|signal| signal.chars().collect::<HashSet<_>>())
            .skip(10)
            .collect_vec();

        Ok(Entry { signals, digits })
    }
}

impl Entry {
    // 0: 6
    // 1: cf, 2
    // 2: 5
    // 3: 5
    // 4: bcdf, 4
    // 5: 5
    // 6: 6
    // 7: acf, 3
    // 8: abcdefg, 7
    // 9: 6

    // len 5: 2 3 5
    // len 6: 0 6 9

    fn count1478(&self) -> i32 {
        self.digits.iter()
            .map(|digit| match digit.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0
            })
            .sum()
    }

    fn deduce(&self) -> Vec<HashSet<char>> {
        let one = self.signals.iter()
            .filter(|s| s.len() == 2)
            .flat_map(|s| s.clone())
            .collect::<HashSet<_>>();
        let four = self.signals.iter()
            .filter(|s| s.len() == 4)
            .flat_map(|s| s.clone())
            .collect::<HashSet<_>>();
        let seven = self.signals.iter()
            .filter(|s| s.len() == 3)
            .flat_map(|s| s.clone())
            .collect::<HashSet<_>>();
        let eight = self.signals.iter()
            .filter(|s| s.len() == 7)
            .flat_map(|s| s.clone())
            .collect::<HashSet<_>>();

        let t = seven.difference(&one).cloned().next().unwrap();
        let m_tl = four.difference(&one).cloned().collect::<HashSet<_>>();
        let b_bl = eight.difference(&seven)
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&four)
            .cloned()
            .collect::<HashSet<_>>();
        let tr_br = one;

        let mut deduced = Vec::new();
        for i in 0..10 {
            deduced.push(HashSet::new());
        }
        for signal in &self.signals {
            let i: usize = match signal.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    match signal.intersection(&tr_br).count() {
                        2 => 3,
                        1 => if signal.intersection(&m_tl).count() == 2 { 5 } else { 2 }
                        _ => panic!(),
                    }
                }
                6 => {
                    match signal.intersection(&tr_br).count() {
                        1 => 6,
                        2 => if signal.intersection(&m_tl).count() == 2 { 9 } else { 0 }
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            };
            deduced[i] = signal.iter()
                .map(|c| c.clone())
                .collect::<HashSet<_>>().clone();
        }

        deduced
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 0)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "}.to_string(), 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}