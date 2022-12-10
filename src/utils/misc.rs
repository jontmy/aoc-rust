use std::{ ops::{ RangeBounds, Bound }, str::FromStr };

use num::{ Num, PrimInt, Integer };
use once_cell_regex::regex;

pub fn get_range_min_max<T, R>(range: R) -> (T, T) where T: Num + Copy, R: RangeBounds<T> {
    let min = match range.start_bound() {
        Bound::Included(min) => *min,
        Bound::Excluded(min) => *min + num::one(),
        Bound::Unbounded => panic!("Range must be lower bounded"),
    };
    let max = match range.end_bound() {
        Bound::Included(max) => *max,
        Bound::Excluded(max) => *max - num::one(),
        Bound::Unbounded => panic!("Range must be upper bounded"),
    };
    (min, max)
}

pub fn get_all_nums<T>(s: &str) -> Vec<T> where T: Integer + FromStr {
    let re = regex!(r"[-\d]+");
    re.captures_iter(s)
        .filter_map(|c| c.get(0).and_then(|m| m.as_str().parse::<T>().ok()))
        .collect()
}