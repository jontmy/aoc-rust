use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use itertools::Itertools;

use crate::utils::coordinates::dim_2::Coordinates;

pub fn solve_part_one(input: &String) -> usize {
    input
        .trim()
        .parse::<Image>()
        .unwrap()
        .successor()
        .successor()
        .count_lit()
}

pub fn solve_part_two(input: &String) -> usize {
    let image = input.trim().parse::<Image>().unwrap();

    (0..50)
        .into_iter()
        .fold(image, |img, _| img.successor())
        .count_lit()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pixel {
    Light,
    Dark,
}

impl Into<char> for Pixel {
    fn into(self) -> char {
        match self {
            Pixel::Light => '#',
            Pixel::Dark => '.',
        }
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Pixel::Light,
            '.' => Pixel::Dark,
            _ => panic!("{}", c),
        }
    }
}

struct Image {
    algorithm: Vec<Pixel>,
    pixels: HashMap<Coordinates, Pixel>,
    padding: Pixel,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (algorithm, pixels) = s.split("\n\n").collect_tuple().unwrap();
        let algorithm = algorithm.trim().chars().map(Pixel::from).collect();

        let width = pixels.lines().next().unwrap().chars().count() as i32;
        let height = pixels.lines().count() as i32;

        let pixels = pixels.lines().flat_map(&str::chars).map(Pixel::from);

        let pixels = Coordinates::in_area_col_major(0..width, 0..height)
            .zip(pixels)
            .collect();

        let image = Image {
            algorithm,
            pixels,
            padding: Pixel::Dark,
            x_min: 0,
            x_max: width,
            y_min: 0,
            y_max: height,
        };
        Ok(image)
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sb = String::from("\n");
        for y in self.y_min..self.y_max {
            for x in self.x_min..self.x_max {
                sb.push(self.pixels[&Coordinates::at(x, y)].into())
            }
            sb.push('\n');
        }
        write!(f, "{}", sb)
    }
}

impl Image {
    fn successor_pixel(&self, c: Coordinates) -> Pixel {
        let bin = Coordinates::in_area_col_major(c.x() - 1..c.x() + 2, c.y() - 1..c.y() + 2)
            .map(|c| match self.pixels.get(&c).unwrap_or(&self.padding) {
                Pixel::Light => '1',
                Pixel::Dark => '0',
            })
            .join("");

        let bin = usize::from_str_radix(bin.as_str(), 2).unwrap();
        self.algorithm[bin]
    }

    fn successor(self) -> Image {
        const EXPANSION: i32 = 1;
        let pixels = Coordinates::in_area(
            self.x_min - EXPANSION..self.x_max + EXPANSION,
            self.y_min - EXPANSION..self.y_max + EXPANSION,
        )
        .map(|c| (c, self.successor_pixel(c)))
        .collect();

        let padding = match self.padding {
            Pixel::Light => Pixel::Dark,
            Pixel::Dark => Pixel::Light,
        };

        Image {
            algorithm: self.algorithm,
            pixels,
            padding,
            x_min: self.x_min - EXPANSION,
            x_max: self.x_max + EXPANSION,
            y_min: self.y_min - EXPANSION,
            y_max: self.y_max + EXPANSION,
        }
    }

    fn count_lit(self) -> usize {
        self.pixels
            .values()
            .filter(|&px| *px == Pixel::Light)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
    "}.to_string(), 35)]
    fn test_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 0)]
    fn test_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
