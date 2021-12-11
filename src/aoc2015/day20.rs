use std::thread::sleep;
use std::time::Duration;
use gcd::Gcd;
use itertools::Itertools;

pub fn gcd(m: usize, n: usize) -> usize {
    let mut m = m;
    let mut n = n;
    loop {
        if m == 0 {
            return n;
        } else {
            let old_m = m;
            m = n % m;
            n = old_m;
        }
    }
}

pub fn solve_part_one(input: &String) -> usize {
    let threshold = input.parse::<usize>().unwrap();
    let mut memoized = vec![10_usize];

    for house in 2..threshold / 10 {
        let gcd = gcd(house, house - 1);
        let presents = memoized.last().unwrap() + gcd * 10;

        if presents > threshold {
            return house;
        }
        dbg!((house, house - 1));
        dbg!(gcd);

        memoized.push(presents);
        sleep(Duration::new(1, 0));
    }

    unreachable!();
}

fn presents(house: i32) -> i32 {
    (1..=house).into_iter()
        .filter(|i| house % i == 0)
        .map(|i| 10 * i)
        .sum()
}

pub fn solve_part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{presents, solve_part_two};

    #[rstest]
    #[case(1, 10)]
    #[case(2, 30)]
    #[case(3, 40)]
    #[case(4, 70)]
    #[case(5, 60)]
    #[case(6, 120)]
    #[case(7, 80)]
    #[case(8, 150)]
    #[case(9, 130)]
    fn test_part_one(#[case] input: i32, #[case] expected: i32) {
        assert_eq!(expected, presents(input))
    }

    #[rstest]
    #[case("str", 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}