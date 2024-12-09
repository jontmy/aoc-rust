use std::collections::HashMap;
use std::hash::Hash;

use anyhow::anyhow;
use itertools::Itertools;
use ndarray::Array2;
use num::PrimInt;

pub trait Grid<T, I>
where
    I: PrimInt,
{
    fn get(&self, x: I, y: I) -> Option<&T>;
    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T>;
    fn set(&mut self, x: I, y: I, value: T);

    fn adjacent_neighbor_coords(&self, x: I, y: I) -> [(I, I); 4] {
        [
            (x - I::from(1).unwrap(), y),
            (x + I::from(1).unwrap(), y),
            (x, y - I::from(1).unwrap()),
            (x, y + I::from(1).unwrap()),
        ]
    }

    fn adjacent_neighbors_iter<'a>(&'a self, x: I, y: I) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.adjacent_neighbor_coords(x, y)
            .into_iter()
            .filter_map(|(x, y)| self.get(x, y))
    }

    fn adjacent_neighbors(&self, x: I, y: I) -> Vec<&T> {
        self.adjacent_neighbors_iter(x, y).collect()
    }

    fn indexed_adjacent_neighbors_iter<'a>(
        &'a self,
        x: I,
        y: I,
    ) -> impl Iterator<Item = ((I, I), &'a T)>
    where
        T: 'a,
    {
        self.adjacent_neighbor_coords(x, y)
            .into_iter()
            .filter_map(|(x, y)| self.get(x, y).map(|v| ((x, y), v)))
    }

    fn diagonal_neighbor_coords(&self, x: I, y: I) -> [(I, I); 4] {
        [
            (x - I::from(1).unwrap(), y - I::from(1).unwrap()),
            (x - I::from(1).unwrap(), y + I::from(1).unwrap()),
            (x + I::from(1).unwrap(), y - I::from(1).unwrap()),
            (x + I::from(1).unwrap(), y + I::from(1).unwrap()),
        ]
    }

    fn diagonal_neighbors_iter<'a>(&'a self, x: I, y: I) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.diagonal_neighbor_coords(x, y)
            .into_iter()
            .filter_map(|(x, y)| self.get(x, y))
    }

    fn diagonal_neighbors(&self, x: I, y: I) -> Vec<&T> {
        self.diagonal_neighbors_iter(x, y).collect()
    }

    fn indexed_diagonal_neighbors_iter<'a>(
        &'a self,
        x: I,
        y: I,
    ) -> impl Iterator<Item = ((I, I), &'a T)>
    where
        T: 'a,
    {
        self.diagonal_neighbor_coords(x, y)
            .into_iter()
            .filter_map(|(x, y)| self.get(x, y).map(|v| ((x, y), v)))
    }

    fn all_neighbor_coords(&self, x: I, y: I) -> Vec<(I, I)> {
        self.adjacent_neighbor_coords(x, y)
            .into_iter()
            .chain(self.diagonal_neighbor_coords(x, y).into_iter())
            .collect_vec()
    }

    fn all_neighbors_iter<'a>(&'a self, x: I, y: I) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.adjacent_neighbors_iter(x, y)
            .chain(self.diagonal_neighbors_iter(x, y))
    }

    fn all_neighbors(&self, x: I, y: I) -> Vec<&T> {
        self.all_neighbors_iter(x, y).collect()
    }

    fn indexed_all_neighbors_iter<'a>(&'a self, x: I, y: I) -> impl Iterator<Item = ((I, I), &'a T)>
    where
        T: 'a,
    {
        self.indexed_adjacent_neighbors_iter(x, y)
            .chain(self.indexed_diagonal_neighbors_iter(x, y))
    }
}

#[derive(Debug)]
pub struct DenseGrid<T> {
    grid: Array2<T>,
}

impl<T, I> Grid<T, I> for DenseGrid<T>
where
    I: PrimInt,
{
    fn get(&self, x: I, y: I) -> Option<&T> {
        if (x < I::zero()) || (y < I::zero()) {
            return None;
        }
        self.grid.get((
            x.to_usize()
                .expect("should have been able to convert `x` to usize"),
            y.to_usize()
                .expect("should have been able to convert `y` to usize"),
        ))
    }

    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T> {
        self.grid.get_mut((
            x.to_usize()
                .expect("should have been able to convert `x` to usize"),
            y.to_usize()
                .expect("should have been able to convert `y` to usize"),
        ))
    }

    fn set(&mut self, x: I, y: I, value: T) {
        self.grid[[
            x.to_usize()
                .expect("should have been able to convert `x` to usize"),
            y.to_usize()
                .expect("should have been able to convert `y` to usize"),
        ]] = value;
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

impl<T> DenseGrid<T> {
    pub fn as_ndarray(&self) -> &Array2<T> {
        &self.grid
    }
}

#[derive(Debug)]
pub struct SparseGrid<T, I = i32>
where
    I: PrimInt + Hash + Eq,
{
    grid: HashMap<(I, I), T>,
}

impl<T, I> SparseGrid<T, I>
where
    I: PrimInt + Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    fn remove(&mut self, x: I, y: I) -> Option<T> {
        self.grid.remove(&(x, y))
    }
}

impl<T, I> Grid<T, I> for SparseGrid<T, I>
where
    I: PrimInt + Hash + Eq,
{
    fn get(&self, x: I, y: I) -> Option<&T> {
        self.grid.get(&(x, y))
    }

    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T> {
        self.grid.get_mut(&(x, y))
    }

    fn set(&mut self, x: I, y: I, value: T) {
        self.grid.insert((x, y), value);
    }
}
