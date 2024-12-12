use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::v2::{
    grid::{DenseGrid, Grid, GridSearch},
    solver,
};

type Region = HashSet<(i32, i32)>;
type Regions = HashMap<(i32, i32), Region>;

pub struct Solver;

impl Solver {
    pub fn parse_input(&self, input: &str) -> (DenseGrid<char, i32>, Regions) {
        // Pad the char grid with '#' so that we can compute the perimeter and take 2x2 windows later.
        let grid = DenseGrid::<char, i32>::try_from(input)
            .unwrap()
            .pad_into(1, '#');
        let mut visited = HashSet::<(i32, i32)>::new();
        let mut regions = HashMap::new();

        // Get all non-overlapping regions.
        for ((sx, sy), _) in grid.as_ndarray().indexed_iter() {
            let (sx, sy) = (sx as i32, sy as i32);
            let s = (sx, sy);
            if visited.contains(&s) {
                continue;
            }
            let v = *grid.get(sx, sy).unwrap();
            if v == '#' {
                continue;
            }

            let region = grid.bfs_flood_fill(s, |(x, y), _| {
                grid.indexed_adjacent_neighbors_iter(x, y)
                    .filter(|(_, &nv)| nv == v)
                    .map(|((nx, ny), _)| (nx, ny))
                    .collect()
            });
            visited.extend(&region);
            regions.insert(s, region);
        }

        (grid, regions)
    }
}

impl solver::Solver<2024, 12> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let (grid, regions) = self.parse_input(input);
        let mut result = 0;

        for (s, region) in regions {
            let v = *grid.get_from_coords(s.into()).unwrap();
            let area = region.len();

            // Using the padding trick, '#' counts as a plot which does not belong in the same region.
            let perimeter = region
                .into_iter()
                .map(|(x, y)| {
                    grid.indexed_adjacent_neighbors_iter(x, y)
                        .filter(|(_, &nv)| nv != v)
                        .count()
                })
                .sum::<usize>();

            result += area * perimeter;
        }
        result
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let (grid, regions) = self.parse_input(input);
        let mut result = 0;

        // Recolor the regions in the grid so that non-overlapping regions have different letters,
        // otherwise they could be overlapping when taking 2x2 windows over the grid.
        let mapping = regions
            .iter()
            .enumerate()
            .flat_map(|(i, (_, region))| region.iter().map(move |(x, y)| ((*x, *y), i + 1)))
            .collect::<HashMap<_, _>>();
        let grid = grid.indexed_map_into(|(x, y), v| if v == '#' { 0 } else { mapping[&(x, y)] });

        for (s, region) in regions {
            let v = *grid.get_from_coords(s.into()).unwrap();
            let subgrid = grid
                .to_minimum_spanning_subgrid(&region.iter().copied().collect_vec())
                .pad_into(1, 0);

            // Take 2x2 windows so that the we can use the center point of the window to check if the
            // window contains a corner (or even two corners - see comments below).
            let corners = subgrid
                .as_ndarray()
                .windows([2, 2])
                .into_iter()
                .map(|window| {
                    let in_same_region = window
                        .indexed_iter()
                        .filter(|(_, &wv)| wv == v)
                        .collect_vec();
                    match in_same_region.len() {
                        // A A
                        // B A
                        // (1 corner)
                        1 | 3 => 1,
                        // A A
                        // A A
                        // (no corners)
                        0 | 4 => 0,
                        // A B          A A
                        // B A    or    B B
                        // (2 corners) (no corners)
                        2 => {
                            let (x1, y1) = in_same_region[0].0;
                            let (x2, y2) = in_same_region[1].0;
                            if x1 != x2 && y1 != y2 {
                                2
                            } else {
                                0
                            }
                        }
                        _ => unreachable!(),
                    }
                })
                .sum::<usize>();

            let area = region.len();
            result += area * corners;
        }

        result
    }
}
