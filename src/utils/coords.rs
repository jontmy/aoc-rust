use std::{ hash::Hash, ops::{ Add, Sub, Mul }, fmt::{ Display, Formatter } };
use num::{ Num, Integer, PrimInt, Signed };

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coordinates<T> where T: Num {
    x: T,
    y: T,
}

impl<T> Coordinates<T> where T: Num + Copy {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn up(&self) -> Self {
        Self { x: self.x, y: self.y + num::one() }
    }

    pub fn down(&self) -> Self {
        Self { x: self.x, y: self.y - num::one() }
    }

    pub fn left(&self) -> Self {
        Self { x: self.x - num::one(), y: self.y }
    }

    pub fn right(&self) -> Self {
        Self { x: self.x + num::one(), y: self.y }
    }

    pub fn adjacent_neighbors(&self) -> impl Iterator<Item = Self> {
        vec![self.up(), self.down(), self.left(), self.right()].into_iter()
    }

    pub fn diagonal_neighbors(&self) -> impl Iterator<Item = Self> {
        vec![
            self.up().left(),
            self.up().right(),
            self.down().left(),
            self.down().right()
        ].into_iter()
    }

    pub fn all_neighbors(&self) -> impl Iterator<Item = Self> {
        self.adjacent_neighbors().chain(self.diagonal_neighbors())
    }
}

impl<T> Coordinates<T> where T: Num + Signed + Copy {
    pub fn manhattan_distance(&self, other: Self) -> T {
        num::abs_sub(self.x, other.x) + num::abs_sub(self.y, other.y)
    }

    pub fn euclidean_distance(&self, other: Self) -> T {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }
}

impl<T> Add for Coordinates<T> where T: Num + Copy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Coordinates<T> where T: Num + Copy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y + rhs.y)
    }
}

impl<T> Mul for Coordinates<T> where T: Num + Copy {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Display for Coordinates<T> where T: Num + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}