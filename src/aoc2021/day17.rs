use std::collections::HashSet;
use itertools::Itertools;
use serde_scan::scan;
use crate::utils::coordinates::dim_2::Coordinates;

// REFACTOR IN PROGRESS

pub fn solve_part_one(input: &String) -> i32 {
    let (x_min, x_max, y_min, y_max): (i32, i32, i32, i32) = scan!("target area: x={}..{}, y={}..{}" <- input).unwrap();
    let mut highest = 0;
    for dx in 0..x_max {
        for dy in 0..300 {
            let probe = Probe { c: Coordinates::at(0, 0), dx, dy };
            match probe.simulate(x_min, x_max, y_min, y_max) {
                Some(height) => {
                    highest = highest.max(height)
                },
                None => {
                    highest = highest
                },
            }
        }
    }
    highest
}

#[derive(Clone, Copy)]
struct Probe {
    c: Coordinates,
    dx: i32,
    dy: i32,
}

impl Probe {
    fn step(self) -> Self {
        let c = self.c.translate(self.dx, self.dy);
        let dx = if self.dx == 0 { 0 } else if self.dx > 0 { self.dx - 1 } else { self.dx + 1 };
        let dy = self.dy - 1;

        Probe { c, dx, dy }
    }

    fn simulate(&self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Option<i32> {
        let mut probe = self.clone();
        let mut highest = self.c.y();
        loop {
            probe = probe.step();
            highest = highest.max(probe.c.y());
            if probe.c.x() > x_max || probe.c.y() < y_min {
                return None;
            } else if probe.c.x() < x_min && probe.dx == 0 {
                return None;
            } else if probe.c.x() >= x_min && probe.c.x() <= x_max && probe.c.y() >= y_min && probe.c.y() <= y_max {
                return Some(highest);
            }
        }
    }
}



pub fn solve_part_two(input: &String) -> i32 {
    let (x_min, x_max, y_min, y_max): (i32, i32, i32, i32) = scan!("target area: x={}..{}, y={}..{}" <- input).unwrap();
    let mut reached = 0;
    for dx in -1000..1000 {
        for dy in -1000..1000 {
            // dbg!((dx, dy));
            let probe = Probe { c: Coordinates::at(0, 0), dx, dy };
            match probe.simulate(x_min, x_max, y_min, y_max) {
                Some(height) => {
                    reached += 1;
                },
                None => {
                    reached += 0;
                },
            }
        }
    }
    reached
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 0)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
