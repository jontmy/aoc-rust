use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn solve_part_one(input: &String) -> i64 {
    input.lines().into_iter()
        .map(|l| l.parse::<Ingredient>())
        .combinations_with_replacement(100).into_iter()
        .map(|combination| {
            combination.into_iter()
                .map(|ingredient| ingredient.unwrap())
                .reduce(|cookie, ingredient| cookie + ingredient)
                .map(|cookie| cookie.total_score()).unwrap()
        })
        .max().unwrap()
}

pub fn solve_part_two(input: &String) -> i64 {
    0
}

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
        }
        let captures: Captures = RE.captures(s).unwrap();
        let ingredient = Self {
            name: captures.get(1).unwrap().as_str().to_string(),
            capacity: captures.get(2).unwrap().as_str().parse().unwrap(),
            durability: captures.get(3).unwrap().as_str().parse().unwrap(),
            flavor: captures.get(4).unwrap().as_str().parse().unwrap(),
            texture: captures.get(5).unwrap().as_str().parse().unwrap(),
            calories: captures.get(6).unwrap().as_str().parse().unwrap(),
        };
        Ok(ingredient)
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            name: self.name,
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Ingredient {
    fn total_score(self) -> i64 {
        self.capacity.max(0) as i64 *
            self.durability.max(0) as i64 *
            self.flavor.max(0) as i64 *
            self.texture.max(0) as i64
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "}.to_string(), 62842880)]
    fn test_part_one(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("str", 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}