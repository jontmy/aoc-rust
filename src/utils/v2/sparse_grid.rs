use std::collections::HashMap;

pub trait Grid<T> {
    fn new() -> Self;
    fn get(&self, x: i32, y: i32) -> Option<&T>;
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T>;
    fn insert(&mut self, x: i32, y: i32, value: T);
    fn remove(&mut self, x: i32, y: i32) -> Option<T>;

    fn adjacent_neighbors(&self, x: i32, y: i32) -> Vec<&T>;
    fn diagonal_neighbors(&self, x: i32, y: i32) -> Vec<&T>;
    fn all_neighbors(&self, x: i32, y: i32) -> Vec<&T> {
        let mut result = self.adjacent_neighbors(x, y);
        result.extend(self.diagonal_neighbors(x, y));
        result
    }
}

#[derive(Debug)]
pub struct SparseGrid<T> {
    grid: HashMap<(i32, i32), T>,
}

impl<T> Grid<T> for SparseGrid<T> {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.grid.get(&(x, y))
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        self.grid.get_mut(&(x, y))
    }

    fn insert(&mut self, x: i32, y: i32, value: T) {
        self.grid.insert((x, y), value);
    }

    fn remove(&mut self, x: i32, y: i32) -> Option<T> {
        self.grid.remove(&(x, y))
    }

    fn adjacent_neighbors(&self, x: i32, y: i32) -> Vec<&T> {
        let mut result = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 && j != 0 {
                    continue;
                }
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(value) = self.get(x + i, y + j) {
                    result.push(value);
                }
            }
        }
        result
    }

    fn diagonal_neighbors(&self, x: i32, y: i32) -> Vec<&T> {
        let mut result = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 || j == 0 {
                    continue;
                }
                if let Some(value) = self.get(x + i, y + j) {
                    result.push(value);
                }
            }
        }
        result
    }
}
