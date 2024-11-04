use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::utils::advent;

pub struct OldSolver;

type Coordinates = (i32, i32, i32);
type Face = (Coordinates, Coordinates, Coordinates, Coordinates);

impl OldSolver {
    fn faces(cx: Coordinates) -> impl Iterator<Item = Face> {
        let a = vec![(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]; // xy = 1
        let b = vec![(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1)]; // xz = 1
        let c = vec![(0, 0, 0), (0, 1, 0), (0, 0, 1), (0, 1, 1)]; // yz = 1

        let x = vec![(1, 1, 1), (0, 1, 1), (1, 0, 1), (0, 0, 1)]; // xy = 0
        let y = vec![(1, 1, 1), (0, 1, 1), (1, 1, 0), (0, 1, 0)]; // xz = 0
        let z = vec![(1, 1, 1), (1, 0, 1), (1, 1, 0), (1, 0, 0)]; // yz = 0

        vec![a, b, c, x, y, z].into_iter().filter_map(move |face| {
            face.into_iter()
                .map(|f| (cx.0 + f.0, cx.1 + f.1, cx.2 + f.2))
                .sorted()
                .collect_tuple::<Face>()
        })
    }

    fn neighbors(
        cx: Coordinates,
        x_min: i32,
        x_max: i32,
        y_min: i32,
        y_max: i32,
        z_min: i32,
        z_max: i32,
    ) -> HashSet<Coordinates> {
        vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .into_iter()
        .map(|d| (cx.0 + d.0, cx.1 + d.1, cx.2 + d.2))
        .filter(|n| {
            n.0 >= x_min
                && n.0 <= x_max
                && n.1 >= y_min
                && n.1 <= y_max
                && n.2 >= z_min
                && n.2 <= z_max
        })
        .collect()
    }

    fn bfs(cx: Coordinates, cubes: HashSet<Coordinates>) -> HashSet<Coordinates> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let x_max = cubes.iter().map(|c| c.0).max().unwrap() + 1;
        let y_max = cubes.iter().map(|c| c.1).max().unwrap() + 1;
        let z_max = cubes.iter().map(|c| c.2).max().unwrap() + 1;

        queue.push_back(cx);
        while let Some(current) = queue.pop_front() {
            assert!(!cubes.contains(&current));
            visited.insert(cx);
            for neighbor in OldSolver::neighbors(current, 0, x_max, 0, y_max, 0, z_max) {
                if !cubes.contains(&neighbor) && !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
        visited
    }
}

impl advent::OldSolver<2022, 18> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .trim()
            .lines()
            .filter_map(|l| scan_fmt!(l, "{d}, {d}, {d}", i32, i32, i32).ok())
            .flat_map(OldSolver::faces)
            .counts()
            .into_iter()
            .filter(|(_, count)| *count == 1)
            .count()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let cubes = input
            .trim()
            .lines()
            .filter_map(|l| scan_fmt!(l, "{d}, {d}, {d}", i32, i32, i32).ok())
            .collect::<HashSet<_>>();

        let x_max = cubes.iter().map(|c| c.0).max().unwrap() + 1;
        let y_max = cubes.iter().map(|c| c.1).max().unwrap() + 1;
        let z_max = cubes.iter().map(|c| c.2).max().unwrap() + 1;

        let visited = OldSolver::bfs((0, 0, 0), cubes);
        let mut cubes = Vec::new();

        for x in 0..x_max {
            for y in 0..y_max {
                for z in 0..z_max {
                    let c = (x, y, z);
                    if !visited.contains(&c) {
                        cubes.push(c);
                    }
                }
            }
        }

        cubes
            .into_iter()
            .flat_map(OldSolver::faces)
            .counts()
            .into_iter()
            .filter(|(_, count)| *count == 1)
            .count()
    }
}
