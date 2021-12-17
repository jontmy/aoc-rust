use std::str::FromStr;

use itertools::{iterate, Itertools};
use serde_scan::scan;

use crate::utils::coordinates::dim_2::Coordinates;

pub fn solve_part_one(input: &String) -> i32 {
    let target_area = input.parse::<TargetArea>().unwrap();
    (0..=target_area.x_max)
        .into_iter()
        .cartesian_product(-1000..=1000)
        .filter_map(|(dx, dy)| Probe::new(dx, dy).max_height(&target_area))
        .max()
        .unwrap()
}

pub fn solve_part_two(input: &String) -> usize {
    let target_area = input.parse::<TargetArea>().unwrap();
    (0..=target_area.x_max)
        .into_iter()
        .cartesian_product(-1000..=1000)
        .filter_map(|(dx, dy)| Probe::new(dx, dy).max_height(&target_area))
        .count()
}

struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for TargetArea {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (x_min, x_max, y_min, y_max): (i32, i32, i32, i32) =
            scan!("target area: x={}..{}, y={}..{}" <- s).unwrap();

        Ok(TargetArea {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

impl TargetArea {
    fn hit_by(&self, c: Coordinates) -> bool {
        c.x() >= self.x_min && c.x() <= self.x_max && c.y() >= self.y_min && c.y() <= self.y_max
    }
}

#[derive(Debug)]
struct Probe {
    coords: Coordinates,
    dx: i32,
    dy: i32,
}

impl Probe {
    /// Returns a new `Probe` positioned at the origin, given its initial `x-` and `y-`velocities.
    fn new(dx: i32, dy: i32) -> Self {
        Probe {
            coords: Coordinates::at(0, 0),
            dx,
            dy,
        }
    }

    /// Returns the x-component of this `Probe`'s coordinates.
    fn x(&self) -> i32 {
        self.coords.x()
    }

    /// Returns the y-component of this `Probe`'s coordinates.
    fn y(&self) -> i32 {
        self.coords.y()
    }

    /// Returns an infinite iterator of `Probe`s at each step of this `Probe`'s trajectory.
    fn trajectory(self) -> impl Iterator<Item = Probe> {
        iterate(self, |probe| {
            let coordinates = probe.coords.translate(probe.dx, probe.dy);
            let x_velocity = probe.dx - probe.dx.signum();
            let y_velocity = probe.dy - 1;

            Probe {
                coords: coordinates,
                dx: x_velocity,
                dy: y_velocity,
            }
        })
    }

    /// Returns the highest `y`-position reached along this `Probe`'s trajectory if and only if
    /// this `Probe` enters a given target area along said trajectory.
    fn max_height(self, target: &TargetArea) -> Option<i32> {
        let trajectory = self
            .trajectory()
            .take_while(|probe| {
                probe.x() <= target.x_max
                    && probe.y() >= target.y_min
                    && !(probe.dx == 0 && probe.x() < target.x_min)
            })
            .collect_vec();

        let hit_target = trajectory
            .last()
            .map(|probe| target.hit_by(probe.coords))
            .unwrap_or(false);

        return if hit_target {
            trajectory.into_iter().map(|probe| probe.y()).max()
        } else {
            None
        };
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        target area: x=20..30, y=-10..-5
    "}.to_string(), 45)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        target area: x=20..30, y=-10..-5
    "}.to_string(), 112)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
