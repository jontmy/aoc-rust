use std::ops::{Bound, RangeBounds};

use num::Num;

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