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

    let (x_min, x_max) = hv_lines.iter()
        .flat_map(|(x1, _, x2, _)| [x1, x2])
        .minmax().into_option().unwrap();

    let (y_min, y_max) = hv_lines.iter()
        .flat_map(|(_, y1, _, y2)| [y1, y2])
        .minmax().into_option().unwrap();

    let (height, width) = (x_max - x_min + 1, y_max - y_min + 1);
    let grid = vec![vec![0; height]; width];
    hv_lines.iter()
        .fold(grid, |mut grid, (x1, y1, x2, y2)| {
            for y in *y1..=*y2 {
                for x in *x1..=*x2 {
                    grid[y - y_min][x - x_min] += 1;
                }
            }
            grid
        }).iter()
        .flat_map(|row| row)
        .filter(|&freq| *freq >= 2)
        .count()
}

pub fn solve_part_two(input: &String) -> i32 {
    let re = Regex::new(r"(?m)(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let (horizontal_vertical, diagonal): (Vec<_>, Vec<_>) = re.captures_iter(input)
        .map(|capture| {
            let x1 = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y1 = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let x2 = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let y2 = capture.get(4).unwrap().as_str().parse::<usize>().unwrap();
            (x1, y1, x2, y2)
        })
        .partition(|(x1, y1, x2, y2)| *x1 == *x2 || *y1 == *y2);

    let horizontal_vertical = horizontal_vertical.iter()
        .map(|(x1, y1, x2, y2)| (*min(x1, x2), *min(y1, y2), *max(x1, x2), *max(y1, y2)))
        .collect_vec();

    dbg!(&diagonal.len());


    let mut grid = vec![[0; 1000]; 1000];
    for (x1, y1, x2, y2) in horizontal_vertical {
        for y in y1..=y2 {
            for x in x1..=x2 {
                grid[y][x] += 1;
            }
        }
    }

    for (x1, y1, x2, y2) in diagonal {
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

        let coords = xs.iter().zip(ys.iter())
            .for_each(|(x, y)| {
                grid[*y][*x] += 1;
            });
    }

    let mut count = 0;
    for row in grid {
        for cell in row {
            if cell >= 2 {
                count += 1;
            }
        }
    }
    count
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
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}