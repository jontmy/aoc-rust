use super::coords::Coordinates;
use num::{Integer, Num, Signed};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Converts a string slice to a Direction, given an array mapping string slices to possible directions.
    /// Convenience function to avoid typing out match arms.
    pub fn from_str(s: &str, up: &str, down: &str, left: &str, right: &str) -> Self {
        if s == up {
            return Direction::Up;
        } else if s == down {
            return Direction::Down;
        } else if s == left {
            return Direction::Left;
        } else if s == right {
            return Direction::Right;
        } else {
            panic!("Unknown direction: {}", s);
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::Up => "up",
            Direction::Down => "down",
            Direction::Left => "left",
            Direction::Right => "right",
        };
        write!(f, "{s}")
    }
}

impl<T> Into<Coordinates<T>> for Direction
where
    T: Integer + Copy + Signed,
{
    /// Converts this direction into a coordinate unit vector.
    fn into(self) -> Coordinates<T> {
        match self {
            Direction::Up => Coordinates::origin().up(),
            Direction::Down => Coordinates::origin().down(),
            Direction::Left => Coordinates::origin().left(),
            Direction::Right => Coordinates::origin().right(),
        }
    }
}
