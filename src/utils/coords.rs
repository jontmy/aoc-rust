use std::{
    hash::Hash,
    ops::{ Add, Sub, Mul, RangeBounds, Bound },
    fmt::{ Display, Formatter },
    collections::{ HashSet, HashMap, VecDeque },
};
use super::{ directions::Direction, misc };
use map_macro::set;
use num::{
    Num,
    Integer,
    PrimInt,
    Signed,
    CheckedAdd,
    CheckedSub,
    traits::{ WrappingAdd, Euclid, WrappingSub },
    iter::Range,
};

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

/// Utility methods for directional movement of **signed integer coordinates**.
/// For unsigned coordinates, use the `try_*` methods.
impl<T> Coordinates<T> where T: Integer + Signed + Copy {
    /// Increments the `y` coordinate by 1.
    ///
    /// # Example
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).up(), Coordinates::new(0, 1));
    /// ```
    pub fn up(&self) -> Self {
        Self { x: self.x, y: self.y + num::one() }
    }

    /// Increments the `y` coordinate by a given amount.
    /// 
    /// Accepts negative values, but you may want to use `down_by` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).up_by(2), Coordinates::new(0, 2));
    /// assert_eq!(Coordinates::new(0, 0).up_by(-2), Coordinates::new(0, -2));
    /// ```
    pub fn up_by(&self, n: T) -> Self {
        Self { x: self.x, y: self.y + n }
    }

    /// Decrements the `y` coordinate by 1.
    ///
    /// # Example
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).down(), Coordinates::new(0, -1));
    /// ```
    pub fn down(&self) -> Self {
        Self { x: self.x, y: self.y - num::one() }
    }

    /// Decrements the `y` coordinate by a given amount.
    /// 
    /// Accepts negative values, but you may want to use `up_by` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).down_by(2), Coordinates::new(0, -2));
    /// assert_eq!(Coordinates::new(0, 0).down_by(-2), Coordinates::new(0, 2));
    /// ```
    pub fn down_by(&self, n: T) -> Self {
        Self { x: self.x, y: self.y - n }
    }

    /// Decrements the `x` coordinate by 1.
    ///
    /// # Example
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).left(), Coordinates::new(-1, 0));
    /// ```
    pub fn left(&self) -> Self {
        Self { x: self.x - num::one(), y: self.y }
    }

    /// Decrements the `x` coordinate by a given amount.
    /// 
    /// Accepts negative values, but you may want to use `right_by` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).left_by(2), Coordinates::new(-2, 0));
    /// assert_eq!(Coordinates::new(0, 0).left_by(-2), Coordinates::new(2, 0));
    /// ```
    pub fn left_by(&self, n: T) -> Self {
        Self { x: self.x - n, y: self.y }
    }

    /// Increments the `x` coordinate by 1.
    ///
    /// # Example
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).right(), Coordinates::new(1, 0));
    /// ```
    pub fn right(&self) -> Self {
        Self { x: self.x + num::one(), y: self.y }
    }

    /// Increments the `x` coordinate by a given amount.
    /// 
    /// Accepts negative values, but you may want to use `left_by` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).right_by(2), Coordinates::new(2, 0));
    /// assert_eq!(Coordinates::new(0, 0).right_by(-2), Coordinates::new(-2, 0));
    /// ```
    pub fn right_by(&self, n: T) -> Self {
        Self { x: self.x + n, y: self.y }
    }

    /// Moves the coordinates in a given direction by 1.
    /// 
    /// Equivalent to calling the appropriate directional `*` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::{coords::Coordinates, directions::Direction};
    /// 
    /// assert_eq!(Coordinates::new(0, 0).step(Direction::Up), Coordinates::new(0, 1));
    /// assert_eq!(Coordinates::new(0, 0).step(Direction::Down), Coordinates::new(0, -1));
    /// assert_eq!(Coordinates::new(0, 0).step(Direction::Left), Coordinates::new(-1, 0));
    /// assert_eq!(Coordinates::new(0, 0).step(Direction::Right), Coordinates::new(1, 0));
    /// ```
    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    /// Moves the coordinates in a given direction by a given amount.
    /// 
    /// Equivalent to calling the appropriate directional `*_by` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::{coords::Coordinates, directions::Direction};
    /// 
    /// assert_eq!(Coordinates::new(0, 0).step_by(Direction::Up, 2), Coordinates::new(0, 2));
    /// assert_eq!(Coordinates::new(0, 0).step_by(Direction::Down, 3), Coordinates::new(0, -3));
    /// assert_eq!(Coordinates::new(0, 0).step_by(Direction::Left, 4), Coordinates::new(-4, 0));
    /// assert_eq!(Coordinates::new(0, 0).step_by(Direction::Right, 5), Coordinates::new(5, 0));
    pub fn step_by(&self, direction: Direction, n: T) -> Self {
        match direction {
            Direction::Up => self.up_by(n),
            Direction::Down => self.down_by(n),
            Direction::Left => self.left_by(n),
            Direction::Right => self.right_by(n),
        }
    }
}

/// Utility methods for *signed or unsigned* **integer coordinates**, useful for checking that a coordinate won't go out of bounds of a grid.
impl<T> Coordinates<T> where T: Integer + Copy + CheckedAdd + CheckedSub {
    pub fn try_up(&self) -> Option<Self> {
        self.y.checked_add(&num::one()).map(|y| Self { x: self.x, y })
    }

    pub fn try_up_by(&self, n: T) -> Option<Self> {
        self.y.checked_add(&n).map(|y| Self { x: self.x, y })
    }

    /// Increments the `y` coordinate by 1, returning `None` if it goes out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).try_bounded_up_by(1, ..=2), Some(Coordinates::new(0, 1)));
    /// assert_eq!(Coordinates::new(0, 2).try_bounded_up_by(1, ..=2), None);
    /// ```
    pub fn try_bounded_up_by<R>(&self, n: T, y_range: R) -> Option<Self> where R: RangeBounds<T> {
        self.y
            .checked_add(&n)
            .filter(|y| y_range.contains(y))
            .map(|y| (self.x, y).into())
    }

    pub fn try_down(&self) -> Option<Self> {
        self.y.checked_sub(&num::one()).map(|y| Self { x: self.x, y })
    }

    pub fn try_down_by(&self, n: T) -> Option<Self> {
        self.y.checked_sub(&n).map(|y| Self { x: self.x, y })
    }

    /// Decrements the `y` coordinate by 1, returning `None` if it goes out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 2).try_bounded_down_by(1, 0..), Some(Coordinates::new(0, 1)));
    /// assert_eq!(Coordinates::new(0, 0).try_bounded_down_by(1, 0..), None);
    /// ```
    pub fn try_bounded_down_by<R>(&self, n: T, y_range: R) -> Option<Self> where R: RangeBounds<T> {
        self.y
            .checked_sub(&n)
            .filter(|y| y_range.contains(y))
            .map(|y| (self.x, y).into())
    }

    pub fn try_right(&self) -> Option<Self> {
        self.x.checked_add(&num::one()).map(|x| Self { x, y: self.y })
    }

    pub fn try_right_by(&self, n: T) -> Option<Self> {
        self.x.checked_add(&n).map(|x| Self { x, y: self.y })
    }

    /// Increments the `x` coordinate by 1, returning `None` if it goes out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).try_bounded_right_by(1, ..=2), Some(Coordinates::new(1, 0)));
    /// assert_eq!(Coordinates::new(2, 0).try_bounded_right_by(1, ..=2), None);
    /// ```
    pub fn try_bounded_right_by<R>(&self, n: T, x_range: R) -> Option<Self> where R: RangeBounds<T> {
        self.x
            .checked_add(&n)
            .filter(|x| x_range.contains(x))
            .map(|x| (x, self.y).into())
    }

    pub fn try_left(&self) -> Option<Self> {
        self.x.checked_sub(&num::one()).map(|x| Self { x, y: self.y })
    }

    pub fn try_left_by(&self, n: T) -> Option<Self> {
        self.x.checked_sub(&n).map(|x| Self { x, y: self.y })
    }

    /// Decrements the `x` coordinate by 1, returning `None` if it goes out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(2, 0).try_bounded_left_by(1, 0..), Some(Coordinates::new(1, 0)));
    /// assert_eq!(Coordinates::new(0, 0).try_bounded_left_by(1, 0..), None);
    /// ```
    pub fn try_bounded_left_by<R>(&self, n: T, x_range: R) -> Option<Self> where R: RangeBounds<T> {
        self.x
            .checked_sub(&n)
            .filter(|x| x_range.contains(x))
            .map(|x| (x, self.y).into())
    }
}

/// Utility methods for *signed or unsigned* **integer coordinates**, useful for wrapping around the edges of a grid.
impl<T> Coordinates<T> where T: Integer + Copy + WrappingAdd + WrappingSub {
    /// Increments the `y` coordinate by 1, setting it to the lower bound of the range if it exceeds the upper bound.
    /// 
    /// Panics if the range is unbounded on either end.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).wrapping_up(0..=2), Coordinates::new(0, 1));
    /// assert_eq!(Coordinates::new(0, 2).wrapping_up(0..=2), Coordinates::new(0, 0));
    /// assert_eq!(Coordinates::new(0, 2).wrapping_up(-2..=2), Coordinates::new(0, -2));
    /// ```
    pub fn wrapping_up<R>(&self, y_range: R) -> Self where R: RangeBounds<T> {
        let (y_min, y_max) = misc::get_range_min_max(y_range);
        let y = self.y.wrapping_add(&num::one());
        let y = Self::get_wrapped(y, y_min, y_max);
        (self.x, y).into()
    }

    /// Decrements the `y` coordinate by 1, setting it to the upper bound of the range if it is lower than the lower bound.
    /// 
    /// Panics if the range is unbounded on either end.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 2).wrapping_down(0..=2), Coordinates::new(0, 1));
    /// assert_eq!(Coordinates::new(0, 0).wrapping_down(0..=2), Coordinates::new(0, 2));
    /// assert_eq!(Coordinates::new(0, -2).wrapping_down(-2..=2), Coordinates::new(0, 2));
    /// ```
    pub fn wrapping_down<R>(&self, y_range: R) -> Self where R: RangeBounds<T> {
        let (y_min, y_max) = misc::get_range_min_max(y_range);
        let y = self.y.wrapping_sub(&num::one());
        let y = Self::get_wrapped(y, y_min, y_max);
        (self.x, y).into()
    }

    /// Increments the `x` coordinate by 1, setting it to the lower bound of the range if it exceeds the upper bound.
    /// 
    /// Panics if the range is unbounded on either end.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(0, 0).wrapping_right(0..=2), Coordinates::new(1, 0));
    /// assert_eq!(Coordinates::new(2, 0).wrapping_right(0..=2), Coordinates::new(0, 0));
    /// assert_eq!(Coordinates::new(2, 0).wrapping_right(-2..=2), Coordinates::new(-2, 0));
    /// ```
    pub fn wrapping_right<R>(&self, x_range: R) -> Self where R: RangeBounds<T> {
        let (x_min, x_max) = misc::get_range_min_max(x_range);
        let x = self.x.wrapping_add(&num::one());
        let x = Self::get_wrapped(x, x_min, x_max);
        (x, self.y).into()
    }

    /// Decrements the `x` coordinate by 1, setting it to the upper bound of the range if it is lower than the lower bound.
    /// 
    /// Panics if the range is unbounded on either end.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// assert_eq!(Coordinates::new(2, 0).wrapping_left(0..=2), Coordinates::new(1, 0));
    /// assert_eq!(Coordinates::new(0, 0).wrapping_left(0..=2), Coordinates::new(2, 0));
    /// assert_eq!(Coordinates::new(-2, 0).wrapping_left(-2..=2), Coordinates::new(2, 0));
    /// ```
    pub fn wrapping_left<R>(&self, x_range: R) -> Self where R: RangeBounds<T> {
        let (x_min, x_max) = misc::get_range_min_max(x_range);
        let x = self.x.wrapping_sub(&num::one());
        let x = Self::get_wrapped(x, x_min, x_max);
        (x, self.y).into()
    }

    fn get_wrapped(v: T, v_min: T, v_max: T) -> T {
        if v > v_max { v_min } else if v < v_min { v_max } else { v }
    }
}

/// Utility methods for getting the neighbors of *signed or unsigned* **integer coordinates**.
impl<T> Coordinates<T> where T: Integer + Copy + CheckedAdd + CheckedSub {
    /// Returns the neighbors of this coordinate which are directly above, below, to the left, or to
    /// the right, in that exact order.
    ///
    /// Returns only the neighbors that are within the bounds of that integer type.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    ///
    /// // With signed integers, all 4 neighbors around the origin are returned.
    /// assert_eq!(Coordinates::new(0, 0).orthogonal_neighbors(), vec![
    ///     Coordinates::new(0, 1),
    ///     Coordinates::new(0, -1),
    ///     Coordinates::new(-1, 0),
    ///     Coordinates::new(1, 0),
    /// ]);
    ///
    /// // With unsigned integers, only the 2 neighbors in the positive quadrant are returned.
    /// assert_eq!(Coordinates::<usize>::new(0, 0).orthogonal_neighbors(), vec![
    ///     Coordinates::new(0, 1),
    ///     Coordinates::new(1, 0),
    /// ]);
    /// ```
    pub fn orthogonal_neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if let Some(up) = self.try_up() {
            neighbors.push(up);
        }
        if let Some(down) = self.try_down() {
            neighbors.push(down);
        }
        if let Some(left) = self.try_left() {
            neighbors.push(left);
        }
        if let Some(right) = self.try_right() {
            neighbors.push(right);
        }
        neighbors
    }

    /// Returns the diagonally-adjacent neighbors of this coordinate in clockwise order, starting
    /// from the neighbor diagonally-adjacent to the top-left corner.
    ///
    /// Returns only the neighbors that are within the bounds of that integer type.
    ///
    /// # Examples
    ///
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    ///
    /// // With signed integers, all 4 corner-adjacent neighbors around the origin are returned.
    /// assert_eq!(Coordinates::new(0, 0).diagonal_neighbors(), vec![
    ///     Coordinates::new(-1, 1),
    ///     Coordinates::new(1, 1),
    ///     Coordinates::new(1, -1),
    ///     Coordinates::new(-1, -1),
    /// ]);
    ///
    /// // With unsigned integers, only the single neighbor in the positive quadrant is returned.
    /// assert_eq!(Coordinates::<usize>::new(0, 0).diagonal_neighbors(), vec![
    ///     Coordinates::new(1, 1),
    /// ]);
    pub fn diagonal_neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if let Some(up_left) = self.try_up().and_then(|up| up.try_left()) {
            neighbors.push(up_left);
        }
        if let Some(up_right) = self.try_up().and_then(|up| up.try_right()) {
            neighbors.push(up_right);
        }
        if let Some(down_right) = self.try_down().and_then(|down| down.try_right()) {
            neighbors.push(down_right);
        }
        if let Some(down_left) = self.try_down().and_then(|down| down.try_left()) {
            neighbors.push(down_left);
        }
        neighbors
    }

    /// Returns any of the neighbors of this coordinate which are edge-adjacent (orthogonal) or corner-adjacent (diagonal).
    /// 
    /// Equivalent to calling `orthogonal_neighbors` and `diagonal_neighbors` and concatenating the results.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// // With signed integers, all 8 neighbors around the origin are returned.
    /// assert_eq!(Coordinates::new(0, 0).all_neighbors().len(), 8);
    /// 
    /// // With unsigned integers, only the 3 neighbors in the positive quadrant are returned.
    /// assert_eq!(Coordinates::<usize>::new(0, 0).all_neighbors().len(), 3);
    /// ```
    pub fn all_neighbors(&self) -> Vec<Self> {
        let mut neighbors = self.orthogonal_neighbors();
        neighbors.extend(self.diagonal_neighbors());
        neighbors
    }
}

/// Utility methods for *signed or unsigned* **integer coordinates** wrapping around bounds, useful for grids with wrap-around.
/// These methods return sets of coordinates instead of vectors, to avoid potential duplicates caused by wrapping-around.
impl<T> Coordinates<T> where T: Integer + Copy + WrappingAdd + WrappingSub + Hash + Eq {
    /// Returns the neighbors of this coordinate which are directly above, below, to the left, or to
    /// the right, wrapping around the given bounds (and of the integer type) as necessary.
    /// 
    /// **Panics** if either of the ranges isn't bounded on both ends.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// // Finding the neighbors of the origin in a 3x3 grid centered around the origin...
    /// let neighbors = Coordinates::new(0, 0)
    ///     .orthogonal_neighbors_wrapping(-1..=1, -1..=1);
    /// 
    /// // ...includes all 4 of the edge-adjacent neighbors.
    /// assert_eq!(neighbors.len(), 4);
    /// assert!(neighbors.contains(&Coordinates::new(0, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(0, -1)));
    /// assert!(neighbors.contains(&Coordinates::new(1, 0)));
    /// assert!(neighbors.contains(&Coordinates::new(-1, 0)));
    /// 
    /// // But if shift the 3x3 grid such that the origin is at the bottom-left corner...
    /// let neighbors = Coordinates::new(0, 0)
    ///     .orthogonal_neighbors_wrapping(0..=2, 0..=2);
    /// 
    /// // ...then the neighbors of the origin include the top-left and bottom-right corner.
    /// assert_eq!(neighbors.len(), 4);
    /// assert!(neighbors.contains(&Coordinates::new(0, 2)));
    /// assert!(neighbors.contains(&Coordinates::new(0, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(1, 0)));
    /// assert!(neighbors.contains(&Coordinates::new(2, 0)));
    /// 
    /// // And if we use a 1x1 grid centered around the origin...
    /// let neighbors = Coordinates::new(0, 0)
    ///     .orthogonal_neighbors_wrapping(0..=0, 0..=0);
    /// 
    /// // ...then there are no neighbors, because even though it would wrap around to the origin,
    /// // the origin is not a neighbor of itself.
    /// assert!(neighbors.is_empty());
    /// 
    /// // Works on unsigned integers too, and the wrapping will always be limited to the positive quadrant,
    /// // and you won't be able to specify negative bounds (but you must still specify the lower bounds).
    /// 
    /// // Let's try it on a 2x2 grid with the origin at the bottom-left corner...
    /// let neighbors = Coordinates::<usize>::new(0, 0)
    ///    .orthogonal_neighbors_wrapping(0..=1, 0..=1);
    /// // .orthogonal_neighbors_wrapping(-1..=1, -1..=1); // this instead won't compile
    /// // .orthogonal_neighbors_wrapping(..=1, ..=1);     // this instead will panic at runtime
    /// 
    /// // ...and we get 2 neighbors.
    /// assert_eq!(neighbors.len(), 2);
    /// assert!(neighbors.contains(&Coordinates::new(0, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(1, 0)));
    /// ```
    pub fn orthogonal_neighbors_wrapping<R>(&self, x_range: R, y_range: R) -> HashSet<Self> where R: RangeBounds<T> + Clone {
        let mut neighbors = set! {
            self.wrapping_up(y_range.clone()),
            self.wrapping_down(y_range),
            self.wrapping_left(x_range.clone()),
            self.wrapping_right(x_range),
        };
        neighbors.remove(self);
        neighbors
    }

    /// Returns the neighbors of this coordinate which are diagonally-adjacent.
    /// 
    /// **Panics** if either of the ranges isn't bounded on both ends.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// // Finding the diagonal neighbors of the origin in a 3x3 grid centered around the origin...
    /// let neighbors = Coordinates::new(0, 0)
    ///     .diagonal_neighbors_wrapping(-1..=1, -1..=1);
    /// 
    /// // ...includes all 4 of the corner neighbors.
    /// assert_eq!(neighbors.len(), 4);
    /// assert!(neighbors.contains(&Coordinates::new(1, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(1, -1)));
    /// assert!(neighbors.contains(&Coordinates::new(-1, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(-1, -1)));
    /// 
    /// // But if shift the 3x3 grid such that the origin is at the bottom-left corner...
    /// let neighbors = Coordinates::new(0, 0)
    ///    .diagonal_neighbors_wrapping(0..=2, 0..=2);
    /// 
    /// // _ X X
    /// // _ X X
    /// // O _ _
    /// 
    /// // ...then the diagonal neighbors of the origin form the pattern above.
    /// assert_eq!(neighbors.len(), 4);
    /// assert!(neighbors.contains(&Coordinates::new(1, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(1, 2)));
    /// assert!(neighbors.contains(&Coordinates::new(2, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(2, 2)));
    /// 
    /// // And if we use a 1x1 grid centered around the origin...
    /// let neighbors = Coordinates::new(0, 0)
    ///    .diagonal_neighbors_wrapping(0..=0, 0..=0);
    /// 
    /// // ...then there are no neighbors, because even though it would wrap around to the origin,
    /// // the origin is not a neighbor of itself.
    /// assert!(neighbors.is_empty());
    /// 
    /// // Works on unsigned integers too, and the wrapping will always be limited to the positive quadrant,
    /// // and you won't be able to specify negative bounds (but you must still specify the lower bounds).
    /// 
    /// // Let's try it on a 2x2 grid with the origin at the bottom-left corner...
    /// let neighbors = Coordinates::<usize>::new(0, 0)
    ///   .diagonal_neighbors_wrapping(0..=1, 0..=1);
    /// 
    /// // X X
    /// // O X
    /// 
    /// // ...and we get 3 neighbors in the pattern above (the origin is not a neighbor of itself).
    /// assert_eq!(neighbors.len(), 3);
    /// assert!(neighbors.contains(&Coordinates::new(1, 0)));
    /// assert!(neighbors.contains(&Coordinates::new(1, 1)));
    /// assert!(neighbors.contains(&Coordinates::new(0, 1)));
    /// ```
    pub fn diagonal_neighbors_wrapping<R>(&self, x_range: R, y_range: R) -> HashSet<Self> where R: RangeBounds<T> + Clone {
        let mut neighbors = set! {
            self.wrapping_up(y_range.clone()).wrapping_left(x_range.clone()),
            self.wrapping_up(y_range.clone()).wrapping_right(x_range.clone()),
            self.wrapping_down(y_range.clone()).wrapping_right(x_range.clone()),
            self.wrapping_down(y_range).wrapping_left(x_range),
        };
        neighbors.remove(self);
        neighbors
    }

    /// Returns the neighbors of this coordinate which are orthogonally- or diagonally-adjacent.
    /// 
    /// Equivalent to calling `orthogonal_neighbors_wrapping` and `diagonal_neighbors_wrapping` and
    /// combining the results.
    /// 
    /// **Panics** if either of the ranges isn't bounded on both ends.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use advent_of_code::utils::coords::Coordinates;
    /// 
    /// // Finding the neighbors of the origin in a 3x3 grid centered around the origin...
    /// let neighbors = Coordinates::new(0, 0)
    ///    .all_neighbors_wrapping(-1..=1, -1..=1);
    /// 
    /// // ...includes all 8 of the neighbors which fully encompass the 3x3 grid.
    /// assert_eq!(neighbors.len(), 8);
    /// ```
    pub fn all_neighbors_wrapping<R>(&self, x_range: R, y_range: R) -> HashSet<Self> where R: RangeBounds<T> + Clone {
        let mut neighbors = self.orthogonal_neighbors_wrapping(x_range.clone(), y_range.clone());
        neighbors.extend(self.diagonal_neighbors_wrapping(x_range, y_range));
        neighbors
    }
}

impl<T> Coordinates<T> where T: Integer + Signed + Copy {
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

    /// Returns the square of the Euclidean distance between two points -
    /// the straight line distance between them.
    pub fn euclidean_distance_squared(&self, destination: Self) -> T {
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

// impl<T> Coordinates<T> where T: Integer + Copy + Hash + Eq {
//     pub fn manhattan_bfs_path(&self, destination: Self, obstacles: &HashSet<Coordinates<T>>) -> Vec<Coordinates<T>> {
//         let mut queue = VecDeque::new();
//         let mut visited = HashSet::new();
//         let mut parents = HashMap::new();
//         queue.push_back(*self);
//         visited.insert(*self);
//         while let Some(current) = queue.pop_front() {
//             if current == destination {
//                 let mut path = vec![destination];
//                 let mut current = destination;
//                 while let Some(parent) = parents.get(&current) {
//                     path.push(*parent);
//                     current = *parent;
//                 }
//                 path.reverse();
//                 return path;
//             }
//             for neighbor in current.all_neighbors() {
//                 if !visited.contains(&neighbor) && !obstacles.contains(&neighbor) {
//                     queue.push_back(neighbor);
//                     visited.insert(neighbor);
//                     parents.insert(neighbor, current);
//                 }
//             }
//         }
//         vec![]
//     }
// }

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

impl<T> From<(T, T)> for Coordinates<T> where T: Num + Copy {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T> From<(&T, T)> for Coordinates<T> where T: Num + Copy {
    fn from((x, y): (&T, T)) -> Self {
        Self::new(*x, y)
    }
}

impl<T> From<(T, &T)> for Coordinates<T> where T: Num + Copy {
    fn from((x, y): (T, &T)) -> Self {
        Self::new(x, *y)
    }
}

impl<T> From<(&T, &T)> for Coordinates<T> where T: Num + Copy {
    fn from((x, y): (&T, &T)) -> Self {
        Self::new(*x, *y)
    }
}

impl<T> Into<(T, T)> for Coordinates<T> where T: Num {
    fn into(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T> Display for Coordinates<T> where T: Num + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}