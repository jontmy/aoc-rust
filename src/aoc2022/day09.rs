use std::str::FromStr;

use crate::utils::{advent, coords::Coordinates, directions::Direction};

use map_macro::{hash_set};
use num_integer::Roots;
use scan_fmt::scan_fmt;

struct Vector {
    direction: Direction,
    displacement: i32,
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let (direction, displacement) = scan_fmt!(l, "{} {}", String, i32).unwrap();
        let direction = Direction::from_str(&direction, "U", "D", "L", "R");
        Ok(Self {
            direction,
            displacement,
        })
    }
}

pub struct Solver;

impl advent::Solver<2022, 9> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let vectors = input.lines().filter_map(|l| l.parse::<Vector>().ok());
        let mut head = Coordinates::origin();
        let mut tail = Coordinates::origin();
        let mut visited = hash_set![tail];

        for Vector {
            direction,
            displacement,
        } in vectors
        {
            let dest = head.step_by(direction, displacement);
            for head in head.manhattan_path(dest) {
                if tail.euclidean_distance_squared(head).sqrt() < 2 {
                    continue;
                }
                tail = tail.euclidean_step(head);
                visited.insert(tail);
            }
            head = dest;
        }
        visited.len()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let vectors = input.lines().filter_map(|l| l.parse::<Vector>().ok());
        let mut rope = vec![Coordinates::origin(); 10];
        let mut visited = hash_set![Coordinates::origin()];

        for Vector {
            direction,
            displacement,
        } in vectors
        {
            let dest = rope[0].step_by(direction, displacement);
            for head in rope[0].manhattan_path(dest) {
                rope[0] = head;
                for i in 1..10 {
                    // rope[i] is the "tail", rope[i-1] is the "head"
                    if rope[i].euclidean_distance_squared(rope[i - 1]).sqrt() < 2 {
                        continue;
                    }
                    rope[i] = rope[i].euclidean_step(rope[i - 1]);
                }
                visited.insert(rope[9]);
            }
        }
        visited.len()
    }
}
