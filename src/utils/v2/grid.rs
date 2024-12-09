use std::collections::HashMap;
use std::hash::Hash;

use itertools::Itertools;
use ndarray::{Array2, Dim, NdIndex};
use num::{Num, ToPrimitive};
use anyhow::anyhow;

pub trait Grid<T, I: Num + Copy> {
    fn get(&self, x: I, y: I) -> Option<&T>;
    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T>;
    fn set(&mut self, x: I, y: I, value: T);

    fn adjacent_neighbors(&self, x: I, y: I) -> Vec<&T>;
    fn diagonal_neighbors(&self, x: I, y: I) -> Vec<&T>;
    fn all_neighbors(&self, x: I, y: I) -> Vec<&T> {
        let mut result = self.adjacent_neighbors(x, y);
        result.extend(self.diagonal_neighbors(x, y));
        result
    }
}

#[derive(Debug)]
pub struct DenseGrid<T> {
    grid: Array2<T>,
}

impl<T, I: Num + Copy + From<u8> + Into<usize>> Grid<T, I> for DenseGrid<T>
where
    (I, I): NdIndex<Dim<[usize; 2]>>,
{
    fn get(&self, x: I, y: I) -> Option<&T> {
        self.grid.get((x, y))
    }

    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T> {
        self.grid.get_mut((x, y))
    }

    fn set(&mut self, x: I, y: I, value: T) {
        self.grid[[x.into(), y.into()]] = value;
    }

    fn adjacent_neighbors(&self, x: I, y: I) -> Vec<&T> {
        [
            (x - I::from(1), y),
            (x + I::from(1), y),
            (x, y - I::from(1)),
            (x, y + I::from(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x, y))
        .collect()
    }

    fn diagonal_neighbors(&self, x: I, y: I) -> Vec<&T> {
        [
            (x - I::from(1), y - I::from(1)),
            (x - I::from(1), y + I::from(1)),
            (x + I::from(1), y - I::from(1)),
            (x + I::from(1), y + I::from(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x, y))
        .collect()
    }
}

impl TryFrom<&str> for DenseGrid<char> {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let grid = s
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let rows = grid.len();
        let cols = grid[0].len();

        if grid.iter().any(|row| row.len() != cols) {
            Err(anyhow!("Rows have different lengths"))
        } else {
            Ok(Self {
                grid: Array2::from_shape_fn((rows, cols), |(i, j)| grid[i][j]),
            })
        }
    }
}

#[derive(Debug)]
pub struct SparseGrid<T, I: Num = i32> {
    grid: HashMap<(I, I), T>,
}

impl<T, I: Num + Hash + Eq + Copy + From<i8>> SparseGrid<T, I> {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    fn remove(&mut self, x: I, y: I) -> Option<T> {
        self.grid.remove(&(x, y))
    }
}

impl<T, I: Num + Hash + Eq + Copy + From<i8>> Grid<T, I> for SparseGrid<T, I> {
    fn get(&self, x: I, y: I) -> Option<&T> {
        self.grid.get(&(x, y))
    }

    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T> {
        self.grid.get_mut(&(x, y))
    }

    fn set(&mut self, x: I, y: I, value: T) {
        self.grid.insert((x, y), value);
    }

    fn adjacent_neighbors(&self, x: I, y: I) -> Vec<&T> {
        [
            (x - I::from(1), y),
            (x + I::from(1), y),
            (x, y - I::from(1)),
            (x, y + I::from(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x, y))
        .collect()
    }

    fn diagonal_neighbors(&self, x: I, y: I) -> Vec<&T> {
        [
            (x - I::from(1), y - I::from(1)),
            (x - I::from(1), y + I::from(1)),
            (x + I::from(1), y - I::from(1)),
            (x + I::from(1), y + I::from(1)),
        ]
        .into_iter()
        .filter_map(|(x, y)| self.get(x, y))
        .collect()
    }
}
