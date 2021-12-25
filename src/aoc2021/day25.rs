use std::collections::HashMap;
use std::fmt::{write, Debug, Formatter};
use std::str::FromStr;

use itertools::Itertools;

use crate::utils::coordinates::dim_2::Coordinates;

pub fn solve_part_one(input: &String) -> usize {
    let mut seafloor = input.parse::<Seafloor>().unwrap();
    let mut steps = 0;
    loop {
        let (movement, moved_east) = seafloor.successor(Region::East);
        let (movement, moved_south) = movement.successor(Region::South);
        steps += 1;
        if !(moved_east || moved_south) {
            break;
        }
        seafloor = movement;
    }
    steps
}

pub fn solve_part_two(input: &String) -> &str {
    "MERRY X'MAS!"
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Region {
    South,
    East,
    Free,
}

impl From<char> for Region {
    fn from(c: char) -> Self {
        match c {
            '>' => Region::East,
            'v' => Region::South,
            '.' => Region::Free,
            _ => panic!(),
        }
    }
}

impl Into<char> for Region {
    fn into(self) -> char {
        match self {
            Region::East => '>',
            Region::South => 'v',
            Region::Free => '.',
        }
    }
}

struct Seafloor {
    grid: HashMap<Coordinates, Region>,
    width: i32,
    height: i32,
}

impl FromStr for Seafloor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().chars().count() as i32;
        let height = s.lines().count() as i32;

        let regions = s.lines().flat_map(|l| l.chars()).map(|c| Region::from(c));

        let grid = Coordinates::in_area_col_major(0..width, 0..height)
            .zip(regions)
            .collect();

        let seafloor = Seafloor {
            grid,
            width,
            height,
        };
        Ok(seafloor)
    }
}

impl Debug for Seafloor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sb = String::from("\n");
        for y in 0..self.height {
            for x in 0..self.width {
                let c = Coordinates::at(x, y);
                let r = *self.grid.get(&c).unwrap_or(&Region::Free);
                sb.push(r.into());
            }
            sb.push('\n');
        }
        write!(f, "{}", sb)
    }
}

impl Seafloor {
    fn successor_region(&self, x: i32, y: i32) -> Coordinates {
        let region = self
            .grid
            .get(&Coordinates::at(x, y))
            .unwrap_or(&Region::Free);
        match region {
            Region::South => {
                let destination = Coordinates::at(x, if y + 1 >= self.height { 0 } else { y + 1 });
                let target = self.grid.get(&destination).unwrap_or(&Region::Free);
                match target {
                    Region::Free => destination,
                    _ => Coordinates::at(x, y),
                }
            }
            Region::East => {
                let destination = Coordinates::at(if x + 1 >= self.width { 0 } else { x + 1 }, y);
                let target = self.grid.get(&destination).unwrap_or(&Region::Free);
                match target {
                    Region::Free => destination,
                    _ => Coordinates::at(x, y),
                }
            }
            Region::Free => panic!(),
        }
    }

    fn successor(self, direction: Region) -> (Seafloor, bool) {
        let mut grid = Coordinates::in_area(0..self.width, 0..self.height)
            .map(|c| (c, Region::Free))
            .collect::<HashMap<Coordinates, Region>>();

        let (moved, movement): (Vec<bool>, Vec<(Coordinates, Region)>) = self
            .grid
            .iter()
            .filter(|(_, &r)| r == direction)
            .map(|(c, r)| {
                let destination = self.successor_region(c.x(), c.y());
                let moved = destination != *c;
                (moved, (destination, *r))
            })
            .unzip();

        let moved = moved.into_iter().any(|b| b);
        grid.extend(self.grid.into_iter().filter(|(_, r)| *r != direction));
        grid.extend(movement);

        let seafloor = Seafloor {
            grid,
            width: self.width,
            height: self.height,
        };

        (seafloor, moved)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
    "}.to_string(), 58)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }
}
