use std::str::FromStr;

use itertools::Itertools;
use once_cell_regex::regex;

struct Address {
    address: String,
    hypernets: Vec<String>,
}

impl FromStr for Address {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = regex!(r"\[(?P<hypernet>\w+)\]");
        let hypernets = regex
            .captures_iter(s)
            .map(|cap| cap["hypernet"].to_string())
            .collect_vec();
        Ok(Address { address: s.to_string(), hypernets })
    }
}

impl Address {
    fn supports_tls(&self) -> bool {
        Address::has_abba(self.address.as_str()) &&
            self.hypernets.iter().all(|hypernet| !Address::has_abba(hypernet.as_str()))
    }

    fn has_abba(seq: &str) -> bool {
        seq.chars().tuple_windows::<(_, _, _, _)>().any(Address::is_abba)
    }

    fn is_abba(sub_seq: (char, char, char, char)) -> bool {
        let (a, b, c, d) = sub_seq;
        a == d && b == c && a != b
    }
}

pub fn solve_part_one(input: String) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Address>().unwrap())
        .filter(Address::supports_tls)
        .count()
}

pub fn solve_part_two(input: String) -> String {
    "".to_string()
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
}