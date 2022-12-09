use std::{ collections::HashSet, str::FromStr };

use crate::utils::{advent, directions::Direction, coords::Coordinates};

use itertools::Itertools;
use num::Signed;
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Vector {
    direction: Direction,
    displacement: i32,
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let (direction, displacement) = scan_fmt!(l, "{} {}", String, i32).unwrap();
        let direction = Direction::from_str(&direction, "U", "D", "L", "R");
        Ok(Self { direction, displacement })
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
        let mut visited = HashSet::new();
        visited.insert(tail);

        for vector in vectors {
            let dest = head.step_by(vector.direction, vector.displacement);
            for head in head.manhattan_path(dest) {
                if tail.all_neighbors().contains(&head) {
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
        let ins = input
            .trim()
            .lines()
            .map(|l| scan_fmt!(l, "{} {}", String, i32).unwrap());

        let mut rope = vec![(0, 0); 10];
        let mut visited = HashSet::new();

        for (dir, dist) in ins {
            let dir = dir.as_str();
            for _ in 0..dist {
                let (x, y) = rope.first().unwrap().clone();
                rope[0] = match dir {
                    "U" => (x, y + 1),
                    "D" => (x, y - 1),
                    "L" => (x - 1, y),
                    "R" => (x + 1, y),
                    _ => panic!(),
                };

                for t in 1..=9 {
                    let h = t - 1;
                    let (x, y) = rope[h];
                    let (i, j) = rope[t];

                    if x == i && y == j {
                        continue;
                    } else if x - i == 2 && y == j {
                        rope[t] = (i + 1, j);
                    } else if x - i == -2 && y == j {
                        rope[t] = (i - 1, j);
                    } else if x == i && y - j == 2 {
                        rope[t] = (i, j + 1);
                    } else if x == i && y - j == -2 {
                        rope[t] = (i, j - 1);
                    } else if (x - i).abs() == 2 || (y - j).abs() == 2 {
                        if x > i && y > j {
                            rope[t] = (i + 1, j + 1);
                        } else if x > i && y < j {
                            rope[t] = (i + 1, j - 1);
                        } else if x < i && y > j {
                            rope[t] = (i - 1, j + 1);
                        } else if x < i && y < j {
                            rope[t] = (i - 1, j - 1);
                        }
                    }
                }
                visited.insert(rope[9]);
            }
        }
        visited.len()
    }
}