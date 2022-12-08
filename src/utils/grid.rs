use std::fmt::Display;
use std::fmt::Debug;
use std::iter::Sum;

use itertools::Itertools;
use num::Num;

use super::coords::Coordinates;

/// perf: uses two-dimensional vecs for extensibility at the cost of some performance
#[derive(Debug, Clone)]
pub struct Grid<T> {
    vec: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(vec: Vec<Vec<T>>) -> Self {
        let height = vec.len();
        let width = vec
            .first()
            .map(|row| row.len())
            .unwrap_or(0);
        assert!(
            vec.iter().all(|row| row.len() == width),
            "All rows must be of equal length"
        );
        Self { vec, height, width }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn area(&self) -> usize {
        self.height * self.width
    }

    pub fn perimeter(&self) -> usize {
        if self.height == 1 {
            return self.width;
        }
        if self.width == 1 {
            return self.height;
        }
        2 * self.height + 2 * self.width - 4 // corners are double-counted
    }

    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.vec[y][x] = val;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.vec[y][x]
    }

    /// Returns an iterator over the y-th row of this grid.
    pub fn row(&self, y: usize) -> impl Iterator<Item = &T> {
        self.vec[y].iter()
    }

    /// Returns an iterator over the x-th column of this grid.
    pub fn col(&self, x: usize) -> impl Iterator<Item = &T> {
        (0..self.height()).map(move |y| self.get(x, y))
    }

    /// Returns a tuple of the cells directly above, at, and below the given coordinates.
    /// The cells are obtained by iterating starting from the given coordinates towards the edges of the grid.
    pub fn col_split_at(&self, x: usize, y: usize) -> (Vec<&T>, &T, Vec<&T>) {
        let above = (0..y)
            .rev()
            .map(|y| self.get(x, y))
            .collect();
        let below = (y + 1..self.height()).map(|y| self.get(x, y)).collect();
        let this = self.get(x, y);
        (above, this, below)
    }

    /// Returns a tuple of the cells directly left, at, and right of the given coordinates.
    /// The cells are obtained by iterating starting from the given coordinates towards the edges of the grid.
    pub fn row_split_at(&self, x: usize, y: usize) -> (Vec<&T>, &T, Vec<&T>) {
        let left = (0..x)
            .rev()
            .map(|x| self.get(x, y))
            .collect();
        let right = (x + 1..self.width()).map(|x| self.get(x, y)).collect();
        let this = self.get(x, y);
        (left, this, right)
    }

    /// Returns an iterator over the rows of this grid.
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.vec.iter().map(|row| row.iter())
    }

    /// Returns an iterator over the columns of this grid.
    pub fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.width()).map(move |x| self.col(x))
    }

    /// Returns an iterator over the coordinates of this grid in row-major order.
    pub fn coords(&self) -> impl Iterator<Item = Coordinates<usize>> {
        (0..self.height()).cartesian_product(0..self.width()).map(|(y, x)| Coordinates::new(x, y))
    }
}

impl<T> Grid<T> where T: Copy {
    pub fn from_value(rows: usize, columns: usize, value: T) -> Self {
        Self { vec: vec![vec![value; columns]; rows], height: rows, width: columns }
    }

    pub fn from_generator<F>(rows: usize, columns: usize, generator: F, default: T) -> Self
        where F: Fn(Coordinates<usize>) -> Option<T>
    {
        let mut res = Self { vec: vec![Vec::new(); rows], height: rows, width: columns };
        for y in 0..rows {
            let row = &mut res.vec[y];
            for x in 0..columns {
                if let Some(gen) = generator(Coordinates::new(x, y)) {
                    row.push(gen);
                }
            }
        }
        res
    }

    /// Maps the given function over the cells of this grid.
    pub fn map<F, U>(self, f: F) -> Grid<U> where F: Fn(T) -> U, U: Copy {
        self.vec
            .into_iter()
            .map(|row|
                row
                    .into_iter()
                    .map(|cell| f(cell))
                    .collect()
            )
            .collect()
    }

    /// Maps the given function over the cells of this grid, given the coordinates of their coordinates in addition to their values.
    pub fn enumerated_map<F, U>(self, f: F) -> Grid<U>
        where F: Fn(Coordinates<usize>, T) -> U, U: Copy
    {
        self.vec
            .into_iter()
            .enumerate()
            .map(|(y, row)|
                row
                    .into_iter()
                    .enumerate()
                    .map(|(x, cell)| f(Coordinates::new(x, y), cell))
                    .collect()
            )
            .collect()
    }

    /// Filters the cells of this grid by the given predicate.
    /// The cells become `Some<T>` if they match the predicate, and `None` otherwise.
    pub fn filter<F>(self, f: F) -> Grid<Option<T>> where F: Fn(&T) -> bool {
        self.map(|cell| if f(&cell) { Some(cell) } else { None })
    }

    /// Shrinks this grid by one cell on each side.
    /// The outermost cells are removed.
    /// The grid must have at least 2 rows and 2 columns.
    pub fn shrink(self) -> Self {
        if self.height() < 2 || self.width() < 2 {
            return Self::new(Vec::new())
        }
        let mut vec = self.vec;
        vec.pop();
        vec.remove(0);
        for row in &mut vec {
            row.pop();
            row.remove(0);
        }
        Self::new(vec)
    }
}

impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let vec = iter.into_iter().collect();
        Grid::new(vec)
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter().flatten().collect::<Vec<_>>().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::vec::IntoIter<&'a T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter().flatten().collect::<Vec<_>>().into_iter()
    }
}

impl<T> Display for Grid<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.vec.is_empty() {
            writeln!(f, "[empty grid]")?;
            return Ok(());
        }
        for row in self.rows() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}