use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

pub fn solve_part_one(input: &String) -> usize {
    let initial = input.parse::<Grid>().unwrap();
    (0..100)
        .into_iter()
        .fold(initial, |grid, _| grid.next())
        .states()
        .get(&State::On)
        .unwrap()
        .clone()
}

pub fn solve_part_two(input: &String) -> usize {
    let initial = input.parse::<Grid>().unwrap().spoil_corners();
    (0..100)
        .into_iter()
        .fold(initial, |grid, _| grid.next().spoil_corners())
        .states()
        .get(&State::On)
        .unwrap()
        .clone()
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum State {
    On,
    Off,
}

struct Grid {
    grid: Vec<Vec<State>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.lines().count();
        let grid = s
            .lines()
            .into_iter()
            .flat_map(|line| line.chars().into_iter())
            .map(|c| match c {
                '.' => State::Off,
                '#' => State::On,
                _ => panic!(),
            })
            .chunks(size)
            .into_iter()
            .map(|chunk| chunk.collect_vec())
            .collect_vec();

        Ok(Grid { grid, size })
    }
}

impl Grid {
    fn neighbors(&self, row: usize, col: usize) -> Vec<State> {
        (row.saturating_sub(1)..=row + 1)
            .flat_map(|row| (col.saturating_sub(1)..=col + 1).map(move |col| (row, col)))
            .filter(|(r, c)| {
                (0..self.size).contains(r)
                    && (0..self.size).contains(c)
                    && !(*r == row && *c == col)
            })
            .map(|(row, col)| self.grid[row][col])
            .collect_vec()
    }

    fn next(self) -> Grid {
        let grid = (0..self.size)
            .flat_map(|row| (0..self.size).map(move |col| (row, col)))
            .map(|(row, col)| {
                // The state a light should have next is based on its current state (on or off),
                // plus the number of neighbors that are on.
                let on = self
                    .neighbors(row, col)
                    .iter()
                    .filter(|&neighbor| *neighbor == State::On)
                    .count();

                // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                match (self.grid[row][col], on) {
                    (State::On, 2) | (State::On, 3) | (State::Off, 3) => State::On,
                    _ => State::Off,
                }
            })
            .chunks(self.size)
            .into_iter()
            .map(|chunk| chunk.collect_vec())
            .collect_vec();

        Grid {
            grid,
            size: self.size,
        }
    }

    fn states(&self) -> HashMap<&State, usize> {
        self.grid.iter().flat_map(|row| row).counts()
    }

    fn spoil_corners(mut self) -> Self {
        self.grid[0][0] = State::On;
        self.grid[0][self.size - 1] = State::On;
        self.grid[self.size - 1][0] = State::On;
        self.grid[self.size - 1][self.size - 1] = State::On;
        self
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_two, Grid, State};

    #[rstest]
    #[case(indoc::indoc ! {"
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    "}.to_string(), 4)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        let initial = input.parse::<Grid>().unwrap();
        let actual = (0..4)
            .into_iter()
            .fold(initial, |grid, _| grid.next())
            .states()
            .get(&State::On)
            .unwrap()
            .clone();
        assert_eq!(expected, actual)
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    "}.to_string(), 17)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        let initial = input.parse::<Grid>().unwrap().spoil_corners();
        let actual = (0..5)
            .into_iter()
            .fold(initial, |grid, _| grid.next().spoil_corners())
            .states()
            .get(&State::On)
            .unwrap()
            .clone();
        assert_eq!(expected, actual)
    }
}
