use std::cmp::{max, min};

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use regex::Regex;

pub fn solve_part_one(input: &String) -> usize {
    let hv_lines = Regex::new(r"(?m)(\d+),(\d+) -> (\d+),(\d+)").unwrap()
        .captures_iter(input)
        .map(|capture| {
            let x1 = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y1 = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let x2 = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let y2 = capture.get(4).unwrap().as_str().parse::<usize>().unwrap();
            (min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2))
        })
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .collect_vec();

    // Determine the dimensions of the 2D vector to be allocated.
    let (x_min, x_max) = hv_lines.iter()
        .flat_map(|(x1, _, x2, _)| [x1, x2])
        .minmax().into_option()
        .map(|(min, max)| (min.clone(), max.clone())).unwrap();
    let (y_min, y_max) = hv_lines.iter()
        .flat_map(|(_, y1, _, y2)| [y1, y2])
        .minmax().into_option()
        .map(|(min, max)| (min.clone(), max.clone())).unwrap();
    let (height, width) = (x_max - x_min + 1, y_max - y_min + 1);

    // Populate the grid with horizontal and vertical lines only.
    let grid = hv_lines.into_iter()
        .fold(vec![vec![0; height]; width], |mut grid, (x1, y1, x2, y2)| {
            for y in y1..=y2 {
                for x in x1..=x2 {
                    grid[y - y_min][x - x_min] += 1;
                }
            }
            grid
        });

    // Count the number of points on the grid at which two or more lines overlap.
    grid.into_iter()
        .flat_map(|row| row)
        .filter(|freq| *freq >= 2)
        .count()
}

pub fn solve_part_two(input: &String) -> usize {
    let lines = Regex::new(r"(?m)(\d+),(\d+) -> (\d+),(\d+)").unwrap()
        .captures_iter(input)
        .map(|capture| {
            let x1 = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y1 = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let x2 = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let y2 = capture.get(4).unwrap().as_str().parse::<usize>().unwrap();
            (x1, y1, x2, y2)
        })
        .collect_vec();

    // Determine the dimensions of the 2D vector to be allocated.
    let (x_min, x_max) = lines.iter()
        .flat_map(|(x1, _, x2, _)| [x1, x2])
        .minmax().into_option()
        .map(|(min, max)| (min.clone(), max.clone())).unwrap();
    let (y_min, y_max) = lines.iter()
        .flat_map(|(_, y1, _, y2)| [y1, y2])
        .minmax().into_option()
        .map(|(min, max)| (min.clone(), max.clone())).unwrap();
    let (height, width) = (x_max - x_min + 1, y_max - y_min + 1);

    // Partition the lines into horizontal/vertical lines and diagonal lines.
    let (hv_lines, diag_lines): (Vec<_>, Vec<_>) = lines.into_iter()
        .partition(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2);

    // Populate the grid with horizontal and vertical lines only.
    let grid = hv_lines.into_iter()
        .fold(vec![vec![0; height]; width], |mut grid, (x1, y1, x2, y2)| {
            for y in y1.min(y2)..=y1.max(y2) {
                for x in x1.min(x2)..=x1.max(x2) {
                    grid[y - y_min][x - x_min] += 1;
                }
            }
            grid
        });

    // Populate the grid with the remaining diagonal lines.
    let grid = diag_lines.into_iter()
        .flat_map(|(x1, y1, x2, y2)| {
            let xs = if x1 <= x2 {
                (min(x1, x2)..=max(x1, x2)).collect_vec()
            } else {
                (min(x1, x2)..=max(x1, x2)).rev().collect_vec()
            };
            let ys = if y1 <= y2 {
                (min(y1, y2)..=max(y1, y2)).collect_vec()
            } else {
                (min(y1, y2)..=max(y1, y2)).rev().collect_vec()
            };
            xs.into_iter()
                .zip(ys.into_iter())
                .collect_vec()
        })
        .fold(grid, |mut grid, (x, y)| {
            grid[y - y_min][x - x_min] += 1;
            grid
        });

    // Count the number of points on the grid at which two or more lines overlap.
    grid.into_iter()
        .flat_map(|row| row)
        .filter(|freq| *freq >= 2)
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "}.to_string(), 5)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "}.to_string(), 12)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_two(&input))
    }
}