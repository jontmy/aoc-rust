use std::cmp::min;
use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    input
        .lines()
        .map(|l| Reindeer::parse(l))
        .map(|reindeer| reindeer.distance_at_nth_second(2503))
        .max()
        .unwrap()
}

pub fn solve_part_two(input: &String) -> i32 {
    let reindeer = input.lines().map(|l| (Reindeer::parse(l))).collect_vec();
    let mut reindeer_points = reindeer
        .iter()
        .map(|r| (r, 0))
        .collect::<HashMap<&Reindeer, i32>>();

    for i in 1..=2503 {
        let farthest_distance = reindeer
            .iter()
            .map(|r| r.distance_at_nth_second(i))
            .max()
            .unwrap();
        reindeer
            .iter()
            .filter(|r| r.distance_at_nth_second(i) == farthest_distance)
            .for_each(|r| *reindeer_points.entry(r).or_insert(0) += 1)
    }
    *reindeer_points.values().max().unwrap()
}

#[derive(Debug, Eq, Hash)]
struct Reindeer {
    name: String,
    speed: i32,
    duration: i32,
    cooldown: i32,
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.speed == other.speed
            && self.duration == other.duration
            && self.cooldown == other.cooldown
    }
}

impl Reindeer {
    fn parse(s: &str) -> Reindeer {
        let capture = Regex::new(r"(\w+) .*? (\d+) .*? (\d+) .*? (\d+)")
            .unwrap()
            .captures_iter(s)
            .next()
            .unwrap();

        Reindeer {
            name: capture.get(1).unwrap().as_str().to_string(),
            speed: capture.get(2).unwrap().as_str().parse().unwrap(),
            duration: capture.get(3).unwrap().as_str().parse().unwrap(),
            cooldown: capture.get(4).unwrap().as_str().parse().unwrap(),
        }
    }

    fn distance_at_nth_second(&self, seconds: i32) -> i32 {
        let cycles = (seconds as f64 / (self.duration + self.cooldown) as f64).floor() as i32;
        let remainder = min(seconds % (self.duration + self.cooldown), self.duration);
        self.speed * (self.duration * cycles + remainder)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    "}.to_string(), 2660)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }
}
