use std::{ str::FromStr, fmt::Display };

use once_cell_regex::regex;

use crate::utils::advent;

struct Rectangle {
    width: usize,
    height: usize,
}

struct Rotation {
    index: usize,
    offset: usize,
}

enum Operation {
    Fill(Rectangle),
    RotateRow(Rotation),
    RotateColumn(Rotation),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fill_regex = regex!(r"rect (?P<width>\d+)x(?P<height>\d+)");
        let rotate_regex = regex!(
            r"rotate (?P<axis>row|column) [x|y]=(?P<index>\d+) by (?P<offset>\d+)"
        );

        if s.starts_with("rect") {
            let captures = fill_regex.captures(s).unwrap();
            let rect = Rectangle {
                width: captures["width"].parse().unwrap(),
                height: captures["height"].parse().unwrap(),
            };
            return Ok(Self::Fill(rect));
        }

        let captures = rotate_regex.captures(s).unwrap();
        let (axis, index, offset) = (
            &captures["axis"],
            captures["index"].parse().unwrap(),
            captures["offset"].parse().unwrap(),
        );
        let rotation = Rotation { index, offset };

        match axis {
            "row" => Ok(Self::RotateRow(rotation)),
            "column" => Ok(Self::RotateColumn(rotation)),
            _ => Err(()),
        }
    }
}

const HEIGHT: usize = 6;
const WIDTH: usize = 50;

struct Screen {
    grid: [[bool; WIDTH]; HEIGHT],
}

impl Screen {
    fn exec(&mut self, op: Operation) {
        match op {
            Operation::Fill(rect) => self.fill(rect),
            Operation::RotateRow(rotation) => self.rotate_row(rotation),
            Operation::RotateColumn(rotation) => self.rotate_col(rotation),
        }
    }

    fn fill(&mut self, rect: Rectangle) {
        for y in 0..rect.height {
            for x in 0..rect.width {
                self.grid[y][x] = true;
            }
        }
    }

    fn count_lit(self) -> usize {
        self.grid
            .into_iter()
            .flat_map(|x| x)
            .filter(|x| *x)
            .count()
    }

    fn rotate_row(&mut self, rotation: Rotation) {
        let tmp = self.grid[rotation.index].clone();
        for x in 0..WIDTH {
            self.grid[rotation.index][(x + rotation.offset).rem_euclid(WIDTH)] = tmp[x];
        }
    }

    fn rotate_col(&mut self, rotation: Rotation) {
        let mut tmp = [false; HEIGHT];
        for y in 0..HEIGHT {
            tmp[y] = self.grid[y][rotation.index];
        }
        for y in 0..HEIGHT {
            self.grid[(y + rotation.offset).rem_euclid(HEIGHT)][rotation.index] = tmp[y];
        }
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                write!(f, "{}", if self.grid[y][x] { '#' } else { '.' }).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

pub struct Solver;

impl advent::Solver<2016, 8> for Solver {
    type Part1 = usize;
    type Part2 = String;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut screen = Screen { grid: [[false; WIDTH]; HEIGHT] };
        input
            .lines()
            .filter_map(|line| line.parse::<Operation>().ok())
            .for_each(|op| screen.exec(op));

        screen.count_lit()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let mut screen = Screen { grid: [[false; WIDTH]; HEIGHT] };
        input
            .lines()
            .filter_map(|line| line.parse::<Operation>().ok())
            .for_each(|op| screen.exec(op));

        format!("\n{}", screen)
    }
}