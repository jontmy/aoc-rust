use std::str::FromStr;

use map_macro::hash_map;
use once_cell_regex::regex;

pub fn get_all_ints_signed(s: &str) -> Vec<i64> {
    let re = regex!(r"-?\d+");
    re.captures_iter(s)
        .filter_map(|c| c.get(0).and_then(|m| m.as_str().parse().ok()))
        .collect()
}

pub fn get_all_ints_unsigned(s: &str) -> Vec<u64> {
    let re = regex!(r"\d+");
    re.captures_iter(s)
        .filter_map(|c| c.get(0).and_then(|m| m.as_str().parse().ok()))
        .collect()
}
