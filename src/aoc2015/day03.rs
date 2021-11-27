use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

pub fn solve_part_one(input: &String) -> i32 {
    // Track the houses which are visited.
    let visits = input.trim()
        .chars()
        .scan(House{ x: 0, y: 0 }, |state, c: char| {
            *state = state.next(c);
            Some(*state)
        });

    // De-duplicated via a HashSet, and add the initial house at (0, 0);
    let mut visited: HashSet<House> = HashSet::from_iter(visits);
    visited.insert(House {x: 0, y: 0});
    visited.len() as i32

    /*
    // Alternative implementation using a frequency table.
    // Track the number of visits to each house.
    let mut frequencies = input.trim()
        .chars()
        .scan(House{ x: 0, y: 0 }, |state, c: char| {
            *state = state.next(c);
            Some(*state)
        })
        .fold(HashMap::new(), |mut freqs, value| -> HashMap<House, i32>{
            *freqs.entry(value).or_insert(0) += 1;
            freqs
        });

    // Add the visit to the first house at (0, 0).
    *frequencies.entry(House {x: 0, y: 0}).or_insert(0) += 1;

    // Return the number of houses visited.
    frequencies.len() as i32
    */
}

pub fn solve_part_two(input: &String) -> i32 {
    0
}

#[derive(Hash, Copy, Clone)]
struct House {
    x: i32,
    y: i32
}

impl Eq for House { }

impl PartialEq<Self> for House {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for House {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl House {
    fn next(&self, c: char) -> House {
        match c {
            '<' => House{ x: self.x - 1, y: self.y },
            '>' => House{ x: self.x + 1, y: self.y },
            '^' => House{ x: self.x, y: self.y + 1 },
            'v' => House{ x: self.x, y: self.y - 1 },
            _ => panic!("Unmatched character {}", c)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc2015::day03::{solve_part_one, solve_part_two};
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    #[case("^^^^", 5)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    /*
    #[rstest]
    #[case(")", 1)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
     */
}