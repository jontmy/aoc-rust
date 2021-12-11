#![allow(dead_code)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use itertools::Itertools;

use super::coords::Coordinates;

pub struct InfiniteGrid<T: Clone> {
    cells: HashMap<Coordinates, T>,
}

impl <T: Clone> InfiniteGrid<T> {
    pub fn new() -> Self {
        InfiniteGrid::<T> {
            cells: HashMap::new(),
        }
    }

    pub fn init_ranged(initial: T, x_lo: i32, x_hi: i32, y_lo: i32, y_hi: i32) -> Self {
        let mut grid = InfiniteGrid::new();
        for x in x_lo..x_hi {
            for y in y_lo..y_hi {
                grid.insert(x, y, initial.clone());
            }
        }
        grid
    }

    pub fn init_sized(initial: T, width: i32, height: i32) -> Self {
        InfiniteGrid::init_ranged(initial, 0, width, 0, height)
    }

    pub fn width(&self) -> i32 {
        self.cells.keys()
            .map(|c| c.x)
            .minmax().into_option()
            .map_or_else(|| 0, |(min, max)| max - min)
    }

    pub fn height(&self) -> i32 {
        self.cells.keys()
            .map(|c| c.y)
            .minmax().into_option()
            .map_or_else(|| 0, |(min, max)| max - min)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.cells.get(&Coordinates::at(x, y))
    }

    pub fn entry(&mut self, x: i32, y: i32) -> Entry<'_, Coordinates, T> {
        self.cells.entry(Coordinates::at(x, y))
    }

    pub fn insert(&mut self, x: i32, y: i32, val: T) -> Option<T> {
        self.cells.insert(Coordinates::at(x, y), val)
    }

    pub fn remove(&mut self, x: i32, y: i32) -> Option<T> {
        self.cells.remove(&Coordinates::at(x, y))
    }

    pub fn map(self, mapper: impl Fn(T) -> T) -> Self {
        let cells = self.cells.into_iter()
            .map(|(k, v)| (k, mapper(v)))
            .collect::<HashMap<_, _>>();

        InfiniteGrid { cells }
    }

    pub fn enum_map(self, mapper: impl Fn(Coordinates, T) -> T) -> Self {
        let cells = self.cells.into_iter()
            .map(|(k, v)| (k, mapper(k, v)))
            .collect::<HashMap<_, _>>();

        InfiniteGrid { cells }
    }

    pub fn filter(self, predicate: impl Fn(&T) -> bool) -> Self {
        let cells = self.cells.into_iter()
            .filter(|(_, v)| predicate(v))
            .collect::<HashMap<_, _>>();

        InfiniteGrid { cells }
    }

    pub fn orthogonal(&self, x: i32, y: i32) -> Vec<(Coordinates, &T)> {
        Coordinates::at(x, y).orthogonal().into_iter()
            .map(|c| (c, self.cells.get(&c)))
            .filter(|(_, o)| o.is_some())
            .map(|(k, v)| (k, v.unwrap()))
            .collect_vec()
    }

    pub fn diagonal(&self, x: i32, y: i32) -> Vec<(Coordinates, &T)> {
        Coordinates::at(x, y).diagonal().into_iter()
            .map(|c| (c, self.cells.get(&c)))
            .filter(|(_, o)| o.is_some())
            .map(|(k, v)| (k, v.unwrap()))
            .collect_vec()
    }

    pub fn adjacent(&self, x: i32, y: i32) -> Vec<(Coordinates, &T)> {
        Coordinates::at(x, y).adjacent().into_iter()
            .map(|c| (c, self.cells.get(&c)))
            .filter(|(_, o)| o.is_some())
            .map(|(k, v)| (k, v.unwrap()))
            .collect_vec()
    }
}

impl <S: Clone> FromIterator<(usize, usize, S)> for InfiniteGrid<S> {
    fn from_iter<I: IntoIterator<Item=(usize, usize, S)>>(iter: I) -> Self {
        let cells =  iter.into_iter()
            .map(|(x, y, v)| (Coordinates::at(x as i32, y as i32), v))
            .collect::<HashMap<_, _>>();

        InfiniteGrid { cells }
    }
}