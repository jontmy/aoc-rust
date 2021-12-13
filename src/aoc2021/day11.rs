use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;

use crate::utils::coordinates::dim_2::Coordinates;

pub fn solve_part_one(input: &String) -> usize {
    let mut cavern = Cavern::from_str(input).unwrap();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += cavern.step();
    }
    flashes
}

pub fn solve_part_two(input: &String) -> i32 {
    let mut cavern = Cavern::from_str(input).unwrap();
    for i in 1..i32::MAX {
        cavern.step();
        if cavern.is_synchronized() {
            return i;
        }
    }
    unreachable!();
}

struct Cavern {
    octopuses: HashMap<Coordinates, u32>,
    height: i32,
    width: i32,
}

impl FromStr for Cavern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count() as i32;
        let width = s.lines().next().unwrap().chars().count() as i32;
        let energies = s
            .lines()
            .flat_map(&str::chars)
            .map(|c| char::to_digit(c, 10).unwrap());

        let octopuses = Coordinates::in_area(0..width, 0..height)
            .zip(energies)
            .collect::<HashMap<Coordinates, u32>>();

        Ok(Cavern {
            octopuses,
            height,
            width,
        })
    }
}

impl Cavern {
    fn step(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.
        for c in Coordinates::in_area(0..self.width, 0..self.height) {
            self.octopuses.entry(c).and_modify(|e| *e += 1);
        }
        let mut flashed = HashSet::new();
        loop {
            // Then, any octopus with an energy level greater than 9 flashes.
            let flashes = Coordinates::in_area(0..self.width, 0..self.height)
                .filter(|c| *self.octopuses.get(c).unwrap() > 9)
                .filter(|c| !flashed.contains(c))
                .collect::<HashSet<Coordinates>>();

            // This increases the energy level of all adjacent octopuses by 1,
            // including octopuses that are diagonally adjacent.
            for c in flashes.iter().flat_map(|c| c.all_offset_by(1)) {
                self.octopuses.entry(c).and_modify(|e| *e += 1);
            }

            // If this causes an octopus to have an energy level greater than 9, it also flashes.
            // This process continues as long as new octopuses keep having their energy level
            // increased beyond 9.
            if flashes.is_empty() {
                break;
            }
            flashed.extend(flashes);
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0,
        // as it used all of its energy to flash.
        for c in flashed.iter() {
            self.octopuses.entry(*c).and_modify(|e| *e = 0);
        }

        // How many total flashes are there after this step?
        flashed.len()
    }

    fn is_synchronized(&self) -> bool {
        self.octopuses.values().all(|e| *e == 0)
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
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
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
