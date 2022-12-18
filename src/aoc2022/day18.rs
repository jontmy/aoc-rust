use std::collections::HashSet;

use itertools::Itertools;
use scan_fmt::scan_fmt;

use crate::utils::advent;

pub struct Solver;

type Coordinates = (i32, i32, i32);
type Face = (Coordinates, Coordinates, Coordinates, Coordinates);

impl Solver {
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
}

impl advent::Solver<2022, 18> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        input
            .trim()
            .lines()
            .filter_map(|l| scan_fmt!(l, "{d}, {d}, {d}", i32, i32, i32).ok())
            .flat_map(Solver::faces)
            .counts()
            .into_iter()
            .filter(|(_, count)| *count == 1)
            .count()
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        0
    }
}
