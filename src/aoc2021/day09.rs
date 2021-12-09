use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;
use ndarray::Array2;

pub fn solve_part_one(input: &String) -> i32 {
    let cave = input.parse::<Cave>().unwrap();
    (0..cave.height).into_iter()
        .flat_map(|row| (0..cave.width).into_iter().map(move |col| (row as i32, col as i32)))
        .map(|(row, col)| cave.risk(row, col))
        .sum()
}

pub fn solve_part_two(input: &String) -> usize {
    let cave = input.parse::<Cave>().unwrap();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut basins = Vec::new();

    for row in 0..cave.height {
        for col in 0..cave.width {
            let coords @ (row, col) = (row as i32, col as i32);
            if !visited.contains(&coords) && cave.height(row, col).unwrap().2 != 9 {
                let basin = cave.basin(row, col);
                visited.extend(&basin);
                basins.push(basin.len());
            }
        }
    }
    basins.into_iter()
        .sorted().rev()
        .take(3)
        .product()
}

struct Cave {
    height: usize,
    width: usize,
    heightmap: Array2<i32>,
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut heightmap = Array2::<i32>::zeros((height, width));

        for (row, line) in s.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                heightmap[[row, col]] = char.to_string().parse::<i32>().unwrap();
            }
        }

        Ok(Cave {
            height,
            width,
            heightmap,
        })
    }
}

impl Cave {
    fn height(&self, row: i32, col: i32) -> Option<(i32, i32, i32)> {
        if row < 0 || row >= self.height as i32 || col < 0 || col >= self.width as i32 {
            None
        } else {
            Some((row, col, self.heightmap[[row as usize, col as usize]]))
        }
    }

    fn orthogonal(&self, row: i32, col: i32) -> Vec<(i32, i32, i32)> {
        [(row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)].into_iter()
            .filter_map(|(row, col)| self.height(row, col))
            .collect_vec()
    }

    fn risk(&self, row: i32, col: i32) -> i32 {
        let (.., center) = self.height(row, col).unwrap();
        let orthogonal = self.orthogonal(row, col);

        if orthogonal.into_iter().all(|(.., ortho)| ortho > center) {
            center + 1
        } else {
            0
        }
    }

    fn basin(&self, row: i32, col: i32) -> HashSet<(i32, i32)> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert((row, col));
        queue.push_back((row, col));

        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            let unvisited = self.orthogonal(r, c).into_iter()
                .filter(|(r, c, h)| *h < 9 && !visited.contains(&(*r, *c)))
                .map(|(r, c, _)| (r, c))
                .collect_vec();

            queue.extend(&unvisited);
            visited.extend(unvisited);
        }
        visited
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "}.to_string(), 15)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "}.to_string(), 1134)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_two(&input))
    }
}