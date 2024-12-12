use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

use anyhow::anyhow;
use itertools::Itertools;
use ndarray::{s, Array2};
use num::PrimInt;

use crate::utils::v2::coords::Coordinates;

type DefaultIndexType = usize;

pub trait Grid<T, I>
where
    I: PrimInt,
{
    fn get(&self, x: I, y: I) -> Option<&T>;
    fn get_mut(&mut self, x: I, y: I) -> Option<&mut T>;

    fn get_from_coords(&self, coords: Coordinates<I>) -> Option<&T> {
        self.get(coords.x(), coords.y())
    }

    fn get_mut_from_coords(&mut self, coords: Coordinates<I>) -> Option<&mut T> {
        self.get_mut(coords.x(), coords.y())
    }

    fn set(&mut self, x: I, y: I, value: T) {
        if let Some(cell) = self.get_mut(x, y) {
            *cell = value;
        }
    }

    fn set_from_coords(&mut self, coords: Coordinates<I>, value: T) {
        self.set(coords.x(), coords.y(), value);
    }

    fn is_in_bounds(&self, x: I, y: I) -> bool;
    fn is_coords_in_bounds(&self, coords: Coordinates<I>) -> bool;

    fn adjacent_neighbor_coords_iter(&self, x: I, y: I) -> impl Iterator<Item = (I, I)> {
        [
            (x - I::from(1).unwrap(), y),
            (x + I::from(1).unwrap(), y),
            (x, y - I::from(1).unwrap()),
            (x, y + I::from(1).unwrap()),
        ]
        .into_iter()
        .filter(|(x, y)| self.is_in_bounds(*x, *y))
    }

    fn adjacent_neighbor_coords(&self, x: I, y: I) -> Vec<(I, I)> {
        self.adjacent_neighbor_coords_iter(x, y).collect()
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
        self.adjacent_neighbor_coords_iter(x, y)
            .filter_map(|(x, y)| self.get(x, y).map(|v| ((x, y), v)))
    }

    fn diagonal_neighbor_coords_iter(&self, x: I, y: I) -> impl Iterator<Item = (I, I)> {
        [
            (x - I::from(1).unwrap(), y - I::from(1).unwrap()),
            (x - I::from(1).unwrap(), y + I::from(1).unwrap()),
            (x + I::from(1).unwrap(), y - I::from(1).unwrap()),
            (x + I::from(1).unwrap(), y + I::from(1).unwrap()),
        ]
        .into_iter()
        .filter(|(x, y)| self.is_in_bounds(*x, *y))
    }

    fn diagonal_neighbor_coords(&self, x: I, y: I) -> Vec<(I, I)> {
        self.diagonal_neighbor_coords_iter(x, y).collect()
    }

    fn diagonal_neighbors_iter<'a>(&'a self, x: I, y: I) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.diagonal_neighbor_coords_iter(x, y)
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

    fn all_neighbor_coords_iter(&self, x: I, y: I) -> impl Iterator<Item = (I, I)> {
        self.adjacent_neighbor_coords_iter(x, y)
            .chain(self.diagonal_neighbor_coords_iter(x, y))
    }

    fn all_neighbor_coords(&self, x: I, y: I) -> Vec<(I, I)> {
        self.all_neighbor_coords_iter(x, y).collect()
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

pub trait GridFind<T, I>: Grid<T, I>
where
    I: PrimInt,
{
    fn find(&self, value: &T) -> Option<(I, I)>;
    fn find_by<F>(&self, f: F) -> Option<(I, I)>
    where
        F: Fn(&T) -> bool;

    fn find_all_iter(&self, value: &T) -> impl Iterator<Item = (I, I)>;
    fn find_all_by_iter<F>(&self, f: F) -> impl Iterator<Item = (I, I)>
    where
        F: Fn(&T) -> bool;

    fn find_all(&self, value: &T) -> Vec<(I, I)> {
        self.find_all_iter(value).collect()
    }
    fn find_all_by<F>(&self, f: F) -> Vec<(I, I)>
    where
        F: Fn(&T) -> bool,
    {
        self.find_all_by_iter(f).collect()
    }
}

pub trait GridSearch<T, I>: Grid<T, I>
where
    T: PartialEq,
    I: PrimInt + Hash,
{
    fn bfs_flood_fill<F>(&self, start: (I, I), neighbors_fn: F) -> HashSet<(I, I)>
    where
        F: Fn((I, I), &T) -> Vec<(I, I)>;

    fn dfs_find_all<F>(&self, value: T, start: (I, I), neighbors_fn: F) -> Vec<(I, I)>
    where
        F: Fn((I, I), &T) -> Vec<(I, I)>;

    fn dfs_find_all_with_repeats<F>(&self, value: T, start: (I, I), neighbors_fn: F) -> Vec<(I, I)>
    where
        F: Fn((I, I), &T) -> Vec<(I, I)>;
}

#[derive(Debug)]
pub struct DenseGrid<T, I = DefaultIndexType> {
    grid: Array2<T>,
    index_type: PhantomData<I>,
}

impl<T, I> Grid<T, I> for DenseGrid<T, I>
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

    fn is_in_bounds(&self, x: I, y: I) -> bool {
        x >= I::zero()
            && y >= I::zero()
            && x < I::from(self.nrows()).unwrap()
            && y < I::from(self.ncols()).unwrap()
    }

    fn is_coords_in_bounds(&self, coords: Coordinates<I>) -> bool {
        self.is_in_bounds(coords.x(), coords.y())
    }
}

impl<T, I> GridFind<T, I> for DenseGrid<T, I>
where
    T: PartialEq,
    I: PrimInt,
{
    fn find(&self, value: &T) -> Option<(I, I)> {
        self.grid
            .indexed_iter()
            .find(|(_, v)| *v == value)
            .map(|((x, y), _)| (I::from(x).unwrap(), I::from(y).unwrap()))
    }

    fn find_by<F>(&self, f: F) -> Option<(I, I)>
    where
        F: Fn(&T) -> bool,
    {
        self.grid
            .indexed_iter()
            .find(|(_, v)| f(v))
            .map(|((x, y), _)| (I::from(x).unwrap(), I::from(y).unwrap()))
    }

    fn find_all_iter(&self, value: &T) -> impl Iterator<Item = (I, I)> {
        self.grid
            .indexed_iter()
            .filter(move |(_, v)| *v == value)
            .map(|((x, y), _)| (I::from(x).unwrap(), I::from(y).unwrap()))
    }

    fn find_all_by_iter<F>(&self, f: F) -> impl Iterator<Item = (I, I)>
    where
        F: Fn(&T) -> bool,
    {
        self.grid
            .indexed_iter()
            .filter(move |(_, v)| f(v))
            .map(|((x, y), _)| (I::from(x).unwrap(), I::from(y).unwrap()))
    }
}

impl<T, I> GridSearch<T, I> for DenseGrid<T, I>
where
    T: PartialEq,
    I: PrimInt + Hash,
{
    fn bfs_flood_fill<F>(&self, start: (I, I), neighbors_fn: F) -> HashSet<(I, I)>
    where
        F: Fn((I, I), &T) -> Vec<(I, I)>,
    {
        {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(start);

            while let Some(coords) = queue.pop_front() {
                if visited.contains(&coords) {
                    continue;
                }
                if let Some(cell) = self.get(coords.0, coords.1) {
                    visited.insert(coords);
                    for neighbor_coords in neighbors_fn(coords, cell) {
                        if let Some(neighbor) = self.get(neighbor_coords.0, neighbor_coords.1) {
                            if visited.contains(&neighbor_coords) {
                                continue;
                            }
                            queue.push_back(neighbor_coords);
                        }
                    }
                }
            }
            visited
        }
    }

    fn dfs_find_all<F>(&self, value: T, start: (I, I), neighbors_fn: F) -> Vec<(I, I)>
    where
        F: Fn((I, I), &T) -> Vec<(I, I)>,
    {
        {
            let mut visited = HashSet::new();
            let mut stack = vec![start];
            let mut result = Vec::new();

            while let Some(coords) = stack.pop() {
                if visited.contains(&coords) {
                    continue;
                }
                if let Some(cell) = self.get(coords.0, coords.1) {
                    visited.insert(coords);
                    for neighbor_coords in neighbors_fn(coords, &cell) {
                        if let Some(neighbor) = self.get(neighbor_coords.0, neighbor_coords.1) {
                            if visited.contains(&neighbor_coords) {
                                continue;
                            }
                            stack.push(neighbor_coords);
                            if *neighbor == value {
                                result.push(neighbor_coords);
                            }
                        }
                    }
                }
            }
            result
        }
    }

    fn dfs_find_all_with_repeats<F>(&self, value: T, start: (I, I), neighbors_fn: F) -> Vec<(I, I)>
    where
        F: Fn((I, I), &T) -> Vec<(I, I)>,
    {
        {
            let mut visited = HashSet::new();
            let mut stack = vec![start];
            let mut result = Vec::new();

            while let Some(coords) = stack.pop() {
                if let Some(cell) = self.get(coords.0, coords.1) {
                    visited.insert(coords);
                    for neighbor_coords in neighbors_fn(coords, &cell) {
                        if let Some(neighbor) = self.get(neighbor_coords.0, neighbor_coords.1) {
                            stack.push(neighbor_coords);
                            if *neighbor == value {
                                result.push(neighbor_coords);
                            }
                        }
                    }
                }
            }
            result
        }
    }
}

impl<I> TryFrom<&str> for DenseGrid<char, I> {
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
                index_type: PhantomData,
            })
        }
    }
}

impl<I> Display for DenseGrid<char, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.rows() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, I> DenseGrid<T, I>
where
    I: PrimInt,
{
    pub fn as_ndarray(&self) -> &Array2<T> {
        &self.grid
    }

    pub fn nrows(&self) -> usize {
        self.grid.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.grid.ncols()
    }

    pub fn map_into<U, F>(self, f: F) -> DenseGrid<U, I>
    where
        F: FnMut(T) -> U,
        T: Clone,
    {
        let grid = self.grid.mapv(f);
        DenseGrid {
            grid,
            index_type: PhantomData,
        }
    }

    pub fn indexed_map_into<U, F>(self, mut f: F) -> DenseGrid<U, I>
    where
        T: Clone,
        F: FnMut((I, I), T) -> U,
    {
        let grid = self
            .grid
            .indexed_iter()
            .map(|((x, y), v)| f((I::from(x).unwrap(), I::from(y).unwrap()), v.clone()))
            .collect_vec();
        let rows = self.nrows();
        let cols = self.ncols();
        let grid = Array2::from_shape_vec((rows, cols), grid).unwrap();
        DenseGrid {
            grid,
            index_type: PhantomData,
        }
    }

    pub fn pad_into(self, amount: usize, value: T) -> DenseGrid<T, I>
    where
        T: Clone,
    {
        let (rows, cols) = (self.nrows(), self.ncols());
        let mut new_grid = Array2::from_elem((rows + 2 * amount, cols + 2 * amount), value);
        new_grid
            .slice_mut(s![amount..amount + rows, amount..amount + cols])
            .assign(&self.grid);
        DenseGrid {
            grid: new_grid,
            index_type: PhantomData,
        }
    }

    pub fn to_subgrid(&self, (x1, y1): (I, I), (x2, y2): (I, I)) -> DenseGrid<T, I>
    where
        T: Clone,
    {
        let (x1, y1, x2, y2) = (
            x1.to_usize().unwrap(),
            y1.to_usize().unwrap(),
            x2.to_usize().unwrap(),
            y2.to_usize().unwrap(),
        );
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        DenseGrid {
            grid: self.grid.slice(s![x1..=x2, y1..=y2]).to_owned(),
            index_type: PhantomData,
        }
    }

    pub fn to_minimum_spanning_subgrid(&self, points: &[(I, I)]) -> DenseGrid<T, I>
    where
        T: Clone,
    {
        let (min_x, min_y) = points.iter().fold(
            (I::max_value(), I::max_value()),
            |(min_x, min_y), (x, y)| (min_x.min(*x), min_y.min(*y)),
        );
        let (max_x, max_y) = points.iter().fold(
            (I::min_value(), I::min_value()),
            |(max_x, max_y), (x, y)| (max_x.max(*x), max_y.max(*y)),
        );
        self.to_subgrid((min_x, min_y), (max_x, max_y))
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

    fn is_in_bounds(&self, x: I, y: I) -> bool {
        true
    }

    fn is_coords_in_bounds(&self, coords: Coordinates<I>) -> bool {
        true
    }
}
