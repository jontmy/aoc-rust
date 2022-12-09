use super::coords::Coordinates;
use num::{ Num, Signed, Integer };

#[derive(Debug)]
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

impl<T> Into<Coordinates<T>> for Direction where T: Integer + Copy + Signed {
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