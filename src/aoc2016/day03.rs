use std::str::FromStr;

use itertools::Itertools;

struct Triangle(i32, i32, i32);

impl FromStr for Triangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides = s
            .split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
            .collect_vec();
        Ok(Triangle(sides[0], sides[1], sides[2]))
    }
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2 && self.0 + self.2 > self.1 && self.1 + self.2 > self.0
    }

    fn from_group(group: itertools::Chunk<std::str::Lines>) -> Vec<Self> {
        let lengths = group
            .flat_map(|line| line.split_ascii_whitespace())
            .map(|length| length.parse().unwrap())
            .collect_vec();
        vec![
            Triangle(lengths[0], lengths[3], lengths[6]),
            Triangle(lengths[1], lengths[4], lengths[7]),
            Triangle(lengths[2], lengths[5], lengths[8]),
        ]
    }
}

pub fn solve_part_one(input: String) -> usize {
    input
        .lines()
        .map(|l| l.parse::<Triangle>().unwrap())
        .filter(|triangle| triangle.is_valid())
        .count()
}

pub fn solve_part_two(input: String) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .flat_map(|group| Triangle::from_group(group))
        .filter(|triangle| triangle.is_valid())
        .count()
}
