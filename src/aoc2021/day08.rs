use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

pub fn solve_part_one(input: &String) -> usize {
    input.lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .map(|entry| entry.count_unique_length_digits())
        .sum()
}

pub fn solve_part_two(input: &String) -> i32 {
    input.lines()
        .map(|line| line.parse::<Entry>().unwrap())
        .map(|entry| entry.value())
        .map(|x| dbg!(x))
        .sum()
}

#[derive(Debug)]
struct Entry {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Each entry consists of ten unique signal patterns, a '|' delimiter, and
        // finally the four digit output value.
        let (input, output) = s.trim()
            .split('|')
            .map(|substring| {
                substring.split_whitespace()
                    .map(|signal| signal.chars().collect::<HashSet<_>>())
                    .collect_vec()
            })
            .collect_tuple().unwrap();

        Ok(Entry { input, output })
    }
}

impl Entry {
    // Returns the number of times that the digits 1, 4, 7, or 8
    // (the digits with unique numbers of segments) appear.
    fn count_unique_length_digits(self) -> usize {
        self.output.into_iter()
            .filter(|signal| {
                match signal.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false
                }
            })
            .count()
    }

    // Returns the decoded four-digit output value.
    fn value(self) -> i32 {
        let mut digit_to_segments = [None; 10];

        // First pass: deduce which input patterns are '1', '4', '7', '8' by virtue of
        // their distinct number of segments.
        for segments in &self.input {
            let digit = match segments.len() { // number of segments
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => continue
            };
            digit_to_segments[digit] = Some(segments);
        }

        // Second pass: deduce which digit each of the remaining input patterns correspond to.
        for segments in &self.input {
            // Get the number of input segments in common with '4', '7', and '8' as a triplet.
            let segments_in_common: (usize, usize, usize) = [4, 7, 8].into_iter()
                .map(|common| {
                    segments.intersection(digit_to_segments[common].unwrap()).count()
                })
                .collect_tuple().unwrap();

            // Match the input pattern based on the number of segments in common with ('4', '7', '8')
            let digit = match segments_in_common {
                (2, 2, 5) => 2, // 1. '2', '3', and '5' all have 5 segments in common with '8'.
                (3, 3, 5) => 3, // 1(a). '2' has 2 segments in common with '4', versus 3 for '3' and '5'.
                (3, 2, 5) => 5, // 1(b). '3' has 3 segments in common with '7', versus 2 for '5'.
                (4, 3, 6) => 9, // 2. '0', '6', and '9' all have 6 segments in common with '8'.
                (3, 3, 6) => 0, // 2(a). '9' has 4 segments in common with '4', versus 3 for '0' and '6'.
                (3, 2, 6) => 6, // 2(b). '0' has 3 segments in common with '7', versus 2 for '6'.
                (_, _, 2) | (_, _, 3) | (_, _, 4) | (_, _, 7) => continue,
                _ => panic!()
            };
            digit_to_segments[digit] = Some(segments);
        }

        // Match each output pattern to the correct input digit based on the deductions above,
        // then parse and return the resulting 4-digit value.
        self.output.into_iter()
            .map(|o| {
                digit_to_segments.iter()
                    .map(|i| i.unwrap())
                    .position(|i| o.len() == i.len() && o.intersection(i).count() == i.len())
                    .unwrap()
            })
            .rev().enumerate()
            .map(|(place, digit)| digit as i32 * 10_i32.pow(place as u32))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

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
    "}.to_string(), 26)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
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
    "}.to_string(), 61229)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}