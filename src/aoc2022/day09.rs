use std::collections::HashSet;

use crate::utils::advent;

use itertools::Itertools;
use num::Signed;
use scan_fmt::scan_fmt;

pub struct Solver;

impl advent::Solver<2022, 9> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let ins = input
            .trim()
            .lines()
            .map(|l| scan_fmt!(l, "{} {}", String, i32).unwrap());

        let mut head = (0, 0);
        let mut tail = (0, 0);
        let mut visited = HashSet::new();

        for (dir, dist) in ins {
            let dir = dir.as_str();
            for _ in 0..dist {
                let (x, y) = head;
                head = match dir {
                    "U" => (x, y + 1),
                    "D" => (x, y - 1),
                    "L" => (x - 1, y),
                    "R" => (x + 1, y),
                    _ => panic!(),
                };

                let (x, y) = head;
                let (i, j) = tail;
                if x == i && y == j {
                    continue;
                } else if x - i == 2 && y == j {
                    tail = (i + 1, j);
                } else if x - i == -2 && y == j {
                    tail = (i - 1, j);
                } else if x == i && y - j == 2 {
                    tail = (i, j + 1);
                } else if x == i && y - j == -2 {
                    tail = (i, j - 1);
                } else if (x - i).abs() == 2 || (y - j).abs() == 2 {
                    if x > i && y > j {
                        tail = (i + 1, j + 1);
                    } else if x > i && y < j {
                        tail = (i + 1, j - 1);
                    } else if x < i && y > j {
                        tail = (i - 1, j + 1);
                    } else if x < i && y < j {
                        tail = (i - 1, j - 1);
                    }
                }
                visited.insert(tail);

                let (i, j) = tail;
                assert!(x.abs_sub(&i) <= 1, "{} {}", x, i);
                assert!(y.abs_sub(&j) <= 1);
            }
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