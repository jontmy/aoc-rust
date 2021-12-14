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
    }
    let freq = initial.into_iter().counts();
    let (min, max) = freq.values().into_iter().minmax().into_option().unwrap();

    *max - *min
}

pub fn solve_part_two(input: &String) -> u64 {
    let polymer = input.lines().next().unwrap().chars().collect_vec();

    // Parse each rule by regex: {char}{char} -> {char}.
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

    // Keep track of the last element since no other element can be added after it -
    // we'll need it later.
    let last_element = polymer.last().unwrap().clone();

    // Get the frequency table of all elements pairwise, casting usize to u64.
    let mut element_pairs = polymer.into_iter()
        .tuple_windows::<(char, char)>()
        .counts().into_iter()
        .map(|(k, v)| (k, v as u64))
        .collect::<HashMap<_, _>>();

    // Repeat the pair insertions 40 times.
    for _ in 0..40 {
        let mut next = HashMap::new();
        // Elements `x` and `y` form a pair `xy`, occurring `n` times.
        for ((x, y), n) in element_pairs.into_iter() {
            if rules.contains_key(&(x, y)) { // pair insertion occurs: `xy` -> `xzy`
                let z = *rules.get(&(x, y)).unwrap();
                *next.entry((x, z)).or_insert(0) += n; // 1. `xy` -> `xz`
                *next.entry((z, y)).or_insert(0) += n; // 2. `xy` -> `zy`
            } else { // pair insertion does not occur: `xy` -> `xy`
                next.insert((x, y), n);
            }
        }
        element_pairs = next; // this block can be replaced by a `fold`, but it'd be less readable
    }

    // Convert `pairs` - currently an (`xy`, `n`) frequency table - to an (`x`, `n`) frequency
    // table - otherwise double counting of every `y` occurs.
    let element_frequencies = element_pairs
        .into_iter()
        .map(|((x, y), n)| (x, n)) // the conversion
        .sorted_by_key(|(a, _)| a.clone())
        .coalesce(|(a, n), (b, m)| {
            // Merge if and only if two adjacent elements `a` and `b` are the same,
            // adding both their frequencies `n` and `m` together.
            // `a` and `b` are guaranteed to be adjacent if they are the same due to `sorted_by_key`
            if a == b {
                Ok((a, n + m))
            } else {
                Err(((a, n), (b, m)))
            }
        });

    // We don't need the entire frequency table, rather just the min and max elements
    // (`a` and `b` respectively) and their frequencies (`min` and `max` respectively).
    let ((a, min), (b, max)) = element_frequencies
        .minmax_by_key(|(_, c)| c.clone())
        .into_option()
        .unwrap();

    // Include the last element if needed, because no pair was formed with it.
    if a == last_element {
        max - min - 1
    } else if b == last_element {
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
    "}.to_string(), 2188189693529)]
    fn test_part_two(#[case] input: String, #[case] expected: u64) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
