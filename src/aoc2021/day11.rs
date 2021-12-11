use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use itertools::Itertools;
use ndarray::{Array2, ArrayBase};

use crate::utils::coordinates;

pub fn solve_part_one(input: &String) -> i32 {
    let mut grid = input.parse::<Grid>().unwrap();

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += grid.increment();
    }

    flashes
}

pub fn solve_part_two(input: &String) -> i32 {
    let mut grid = input.parse::<Grid>().unwrap();
    for i in 1..i32::MAX {
        grid.increment();
        if grid.octopuses.values().all(|i| *i == 0) {
            return i;
        }
    }
    unreachable!();
}

struct Grid {
    octopuses: HashMap<(i32, i32), i32>,
    width: i32,
    height: i32,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octopuses = s
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars().enumerate().map(move |(c, val)| {
                    (
                        (r as i32, c as i32),
                        val.to_string().parse::<i32>().unwrap(),
                    )
                })
            })
            .collect::<HashMap<(i32, i32), i32>>();

        Ok(Grid {
            octopuses,
            width: s.lines().next().unwrap().len() as i32,
            height: s.lines().count() as i32,
        })
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sb = String::from("\n");
        for r in 0..self.height {
            for c in 0..self.width {
                sb.push_str(&*format!("{}", self.octopuses.get(&(r, c)).unwrap()))
            }
            sb.push_str("\n");
        }
        write!(f, "{}", sb)
    }
}

impl Grid {
    fn increment(&mut self) -> i32 {
        for r in 0..self.height {
            for c in 0..self.width {
                self.octopuses.entry((r, c)).and_modify(|val| *val += 1);
            }
        }

        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        let mut threshold = 8;

        while self.octopuses.values().any(|i| *i > threshold) {
            if threshold == 8 {
                threshold += 1
            }
            let mut flash = Vec::new();
            for r in 0..self.height {
                for c in 0..self.width {
                    let val = *self.octopuses.get(&(r, c)).unwrap();
                    if val > 9 {
                        flash.push((r, c));
                    }
                }
            }
            for (r, c) in &flash {
                let val = *self.octopuses.get(&(*r, *c)).unwrap();
                let origin = vec![*r as usize, *c as usize];
                for adj in coordinates::offset_by(&origin, 1) {
                    let coords: (i32, i32) =
                        adj.into_iter().map(|i| i as i32).collect_tuple().unwrap();
                    self.octopuses.entry(coords).and_modify(|i| {
                        if *i != 0 {
                            *i += 1;
                        }
                    });
                }
            }
            for (r, c) in flash {
                self.octopuses.insert((r, c), 0);
                flashed.insert((r, c));
            }
        }
        flashed.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "}.to_string(), 1656)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "}.to_string(), 195)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
