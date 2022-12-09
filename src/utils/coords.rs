use std::{ hash::Hash, ops::{ Add, Sub, Mul }, fmt::{ Display, Formatter } };
use super::directions::Direction;
use num::{ Num, Integer, PrimInt, Signed };

/// A point in 2D space which can either be signed or unsigned.
/// The `x` and `y` fields are private; use the `x()` and `y()` methods to access them.
/// To destructure into a tuple of `x` and `y`, use the `into()` method.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Coordinates<T> where T: Num {
    x: T,
    y: T,
}

impl<T> Coordinates<T> where T: Num + Copy {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(num::zero(), num::zero())
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Coordinates<T> where T: Integer + Signed + Copy {
    pub fn up(&self) -> Self {
        Self { x: self.x, y: self.y + num::one() }
    }

    pub fn up_by(&self, n: T) -> Self {
        Self { x: self.x, y: self.y + n }
    }

    pub fn down(&self) -> Self {
        Self { x: self.x, y: self.y - num::one() }
    }

    pub fn down_by(&self, n: T) -> Self {
        Self { x: self.x, y: self.y - n }
    }

    pub fn left(&self) -> Self {
        Self { x: self.x - num::one(), y: self.y }
    }

    pub fn left_by(&self, n: T) -> Self {
        Self { x: self.x - n, y: self.y }
    }

    pub fn right(&self) -> Self {
        Self { x: self.x + num::one(), y: self.y }
    }

    pub fn right_by(&self, n: T) -> Self {
        Self { x: self.x + n, y: self.y }
    }

    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    pub fn step_by(&self, direction: Direction, n: T) -> Self {
        match direction {
            Direction::Up => self.up_by(n),
            Direction::Down => self.down_by(n),
            Direction::Left => self.left_by(n),
            Direction::Right => self.right_by(n),
        }
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

    /// Returns the Manhattan distance between two points -
    /// the sum of the straight line distances between their x- and y-coordinates.
    pub fn manhattan_distance(&self, destination: Self) -> T {
        num::abs_sub(self.x, destination.x) + num::abs_sub(self.y, destination.y)
    }

    /// Returns the coordinates of the next step along the shortest path that must be traversed to get to a destination.
    /// Steps are of size 1, and prioritize the x-axis first, then the y-axis.
    /// The step taken will be either directly up, down, left, or right, never diagonal.
    /// See also `manhattan_path()` for the full path, and `euclidean_step()` for a diagonal step.
    pub fn manhattan_step(&self, destination: Self) -> Self {
        if self.x < destination.x {
            self.right()
        } else if self.x > destination.x {
            self.left()
        } else if self.y < destination.y {
            self.up()
        } else if self.y > destination.y {
            self.down()
        } else {
            *self
        }
    }

    /// Returns the coordinates along the shortest path that must be traversed to get to a destination.
    /// Traverses the x-axis first, then the y-axis, in unit steps of 1, never taking diagonal steps.
    /// Excludes the starting point.
    pub fn manhattan_path(&self, destination: Self) -> Vec<Coordinates<T>> {
        let mut current = *self;
        let mut steps = vec![];
        while current != destination {
            if current.x < destination.x {
                current = current.right();
            } else if current.x > destination.x {
                current = current.left();
            } else if current.y < destination.y {
                current = current.up();
            } else if current.y > destination.y {
                current = current.down();
            }
            steps.push(current);
        }
        steps
    }

    /// Returns the Euclidean distance between two points -
    /// the straight line distance between them.
    pub fn euclidean_distance(&self, destination: Self) -> T {
        (self.x - destination.x) * (self.x - destination.x) +
            (self.y - destination.y) * (self.y - destination.y)
    }

    /// Returns the coordinates along the shortest path that must be traversed to get to a destination.
    /// Traverses both the x- and y-axes at the same time, in unit steps of 1, allowing diagonal steps.
    /// Excludes the starting point.
    pub fn euclidean_path(&self, destination: Self) -> Vec<Coordinates<T>> {
        let mut current = *self;
        let mut steps = vec![];
        while current != destination {
            if current.x < destination.x {
                current = current.right();
            } else if current.x > destination.x {
                current = current.left();
            }
            if current.y < destination.y {
                current = current.up();
            } else if current.y > destination.y {
                current = current.down();
            }
            steps.push(current);
        }
        steps
    }

    /// Returns the coordinates of the next step along the shortest path that must be traversed to get to a destination.
    /// The step taken will be either directly up, down, left, or right, or a diagonal combination of two of those.
    /// See also `euclidean_path()` for the full path, and `manhattan_step()` for a non-diagonal step.
    pub fn euclidean_step(&self, destination: Self) -> Self {
        let mut current = *self;
        if current.x < destination.x {
            current = current.right();
        } else if current.x > destination.x {
            current = current.left();
        }
        if current.y < destination.y {
            current = current.up();
        } else if current.y > destination.y {
            current = current.down();
        }
        current
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

impl<T> Into<(T, T)> for Coordinates<T> where T: Num {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}