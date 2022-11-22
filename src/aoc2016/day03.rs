use std::str::FromStr;

use itertools::Itertools;

struct Triangle(i32, i32, i32);

impl FromStr for Triangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides = s.split_ascii_whitespace()
            .map(|i| i.parse().unwrap())
            .collect_vec();
        Ok(Triangle(sides[0], sides[1], sides[2]))
    }
}

impl Triangle {
    fn is_valid(&self) -> bool {
        self.0 + self.1 > self.2 && self.0 + self.2 > self.1 && self.1 + self.2 > self.0
    }
}

pub fn solve_part_one(input: String) -> usize {
    input.lines()
        .map(|l| l.parse::<Triangle>().unwrap())
        .filter(|triangle| triangle.is_valid())
        .count()
}

pub fn solve_part_two(input: String) -> String {
    "".to_string()
}