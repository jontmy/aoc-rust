use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
use once_cell_regex::regex;

struct Address {
    supernets: Vec<String>, // the characters outside brackets []
    hypernets: Vec<String>, // the characters inside brackets []
}

impl FromStr for Address {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hypernet_regex = regex!(r"\[(?P<hypernet>\w+)\]");
        let hypernets = hypernet_regex
            .captures_iter(s)
            .map(|cap| cap["hypernet"].to_string())
            .collect();

        let supernets = hypernet_regex
            .split(s)
            .map(|supernet| supernet.to_string())
            .collect();

        Ok(Address {
            supernets,
            hypernets,
        })
    }
}

impl Address {
    fn supports_tls(&self) -> bool {
        self.supernets
            .iter()
            .any(|hypernet| Address::has_abba(hypernet.as_str()))
            && self
                .hypernets
                .iter()
                .all(|hypernet| !Address::has_abba(hypernet.as_str()))
    }

    fn has_abba(seq: &str) -> bool {
        seq.chars()
            .tuple_windows::<(_, _, _, _)>()
            .any(Address::is_abba)
    }

    fn is_abba(sub_seq: (char, char, char, char)) -> bool {
        let (a, b, c, d) = sub_seq;
        a == d && b == c && a != b
    }

    fn supports_ssl(&self) -> bool {
        let babs = self
            .hypernets
            .iter()
            .flat_map(Address::get_babs)
            .collect::<HashSet<(_, _, _)>>();

        self.supernets
            .iter()
            .flat_map(Address::get_babs)
            .map(|(a, b, _)| (b, a, b)) // convert the ABA into a BAB to search for
            .any(|bab| babs.contains(&bab))
    }

    fn get_babs(seq: &String) -> Vec<(char, char, char)> {
        seq.chars()
            .into_iter()
            .tuple_windows::<(_, _, _)>()
            .filter(|s| Address::is_aba_or_bab(*s))
            .collect()
    }

    fn is_aba_or_bab(sub_seq: (char, char, char)) -> bool {
        let (a, b, c) = sub_seq;
        a == c && b != a
    }
}

pub fn solve_part_one(input: String) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Address>().unwrap())
        .filter(Address::supports_tls)
        .count()
}

pub fn solve_part_two(input: String) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Address>().unwrap())
        .filter(|addr| addr.supports_ssl())
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Address;

    #[rstest]
    #[case("abba[mnop]qrst".to_string(), true)]
    #[case("abcd[bddb]xyyx".to_string(), false)]
    #[case("aaaa[qwer]tyui".to_string(), false)]
    #[case("ioxxoj[asdfgh]zxcvbn".to_string(), true)]
    fn test_address_supports_tls(#[case] input: String, #[case] expected: bool) {
        assert_eq!(input.parse::<Address>().unwrap().supports_tls(), expected)
    }

    #[rstest]
    #[case("aba[bab]xyz".to_string(), true)]
    #[case("xyx[xyx]xyx".to_string(), false)]
    #[case("aaa[kek]eke".to_string(), true)]
    #[case("zazbz[bzb]cdb".to_string(), true)]
    fn test_address_supports_ssl(#[case] input: String, #[case] expected: bool) {
        assert_eq!(input.parse::<Address>().unwrap().supports_ssl(), expected)
    }
}
