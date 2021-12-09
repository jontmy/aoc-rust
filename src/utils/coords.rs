#![allow(dead_code)]

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn at(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }

    pub fn orthogonal(&self) -> Vec<Coordinates> {
        [
            Coordinates::at(0, 1),
            Coordinates::at(0, -1),
            Coordinates::at(1, 0),
            Coordinates::at(-1, 0),
        ]
            .into_iter()
            .map(|delta| *self + delta)
            .collect_vec()
    }

    pub fn diagonal(&self) -> Vec<Coordinates> {
        [
            Coordinates::at(1, 1),
            Coordinates::at(1, -1),
            Coordinates::at(-1, 1),
            Coordinates::at(-1, -1),
        ]
            .into_iter()
            .map(|delta| *self + delta)
            .collect_vec()
    }

    pub fn adjacent(&self) -> Vec<Coordinates> {
        [
            Coordinates::at(0, 1),
            Coordinates::at(0, -1),
            Coordinates::at(1, 0),
            Coordinates::at(-1, 0),
            Coordinates::at(1, 1),
            Coordinates::at(1, -1),
            Coordinates::at(-1, 1),
            Coordinates::at(-1, -1),
        ]
            .into_iter()
            .map(|delta| *self + delta)
            .collect_vec()
    }

    pub fn manhattan(&self, other: Coordinates) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn euclidean(&self, other: Coordinates) -> f64 {
        f64::sqrt(((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64)
    }
}

impl FromStr for Coordinates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let (x, y) = RE.captures_iter(s)
            .take(2)
            .map(|i| i.get(0).unwrap().as_str().parse::<i32>().unwrap())
            .collect_tuple().unwrap();

        Ok(Coordinates { x, y })
    }
}

impl Clone for Coordinates {
    fn clone(&self) -> Self {
        Coordinates::at(self.x, self.y)
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Coordinates {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}