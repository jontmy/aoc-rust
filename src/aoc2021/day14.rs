use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;
use serde_scan::scan;

pub fn solve_part_one(input: &String) -> usize {
    let mut initial = input.lines().next().unwrap().chars().collect_vec();
    let rules = Regex::new(r"(\w)(\w) -> (\w)")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let a = c.get(1).unwrap().as_str().chars().next().unwrap();
            let b = c.get(2).unwrap().as_str().chars().next().unwrap();
            let c = c.get(3).unwrap().as_str().chars().next().unwrap();
            (a, b, c)
        })
        .collect_vec();

    for _ in 0..10 {
        let last = initial.last().unwrap().clone();
        initial = initial
            .into_iter()
            .tuple_windows::<(char, char)>()
            .flat_map(|(a, b)| {
                for (x, y, r) in &rules {
                    if a == *x && b == *y {
                        return [a, *r];
                    }
                }
                return [a, ' '];
            })
            .chain([last])
            .collect_vec();

        // dbg!(&initial);
    }
    let freq = initial.into_iter().counts();
    let (min, max) = freq.values().into_iter().minmax().into_option().unwrap();

    *max - *min
}

pub fn solve_part_two(input: &String) -> i64 {
    let initial = input.lines().next().unwrap().chars().collect_vec();
    let rules = Regex::new(r"(\w)(\w) -> (\w)")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let a = c.get(1).unwrap().as_str().chars().next().unwrap();
            let b = c.get(2).unwrap().as_str().chars().next().unwrap();
            let c = c.get(3).unwrap().as_str().chars().next().unwrap();
            ((a, b), c)
        })
        .collect::<HashMap<(char, char), char>>();

    let last = initial.last().unwrap().clone();

    let pairs = initial.into_iter().tuple_windows::<(char, char)>().counts();

    let mut pairs = pairs
        .into_iter()
        .map(|(k, v)| (k, v as i64))
        .collect::<HashMap<_, _>>();

    for _ in 0..40 {
        let mut next = HashMap::new();

        for ((x, y), c) in pairs.into_iter() {
            if rules.contains_key(&(x, y)) {
                let r = *rules.get(&(x, y)).unwrap();
                *next.entry((x, r)).or_insert(0) += c;
                *next.entry((r, y)).or_insert(0) += c;
            } else {
                next.insert((x, y), c);
            }
        }
        pairs = next;
    }

    let ((a, min), (b, max)) = pairs
        .into_iter()
        .map(|((a, b), c)| (a, c))
        .sorted_by_key(|(a, _)| a.clone())
        .coalesce(|(a, c), (b, d)| {
            if a == b {
                Ok((a, c + d))
            } else {
                Err(((a, c), (b, d)))
            }
        })
        .minmax_by_key(|(_, c)| c.clone())
        .into_option()
        .unwrap();

    if a == last {
        max - min - 1
    } else if b == last {
        max - min + 1
    } else {
        max - min
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "}.to_string(), 1588)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "}.to_string(), 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
