use std::{collections::BTreeMap, vec};

use itertools::Itertools;
use once_cell_regex::regex;

use crate::utils::aoc;

struct Marker {
    // in the format (<grab>x<repeat>)
    start: usize, // index in the input string of the opening parentheses `(`
    end: usize,   // index of the next character after the closing parentheses `)`
    grab: usize,
    repeat: usize,
}

impl Marker {
    fn new(index: regex::Match, grab: &str, repeat: &str) -> Self {
        Marker {
            start: index.start(),
            end: index.start() + "(x)".len() + grab.len() + repeat.len(),
            grab: grab.parse().unwrap(),
            repeat: repeat.parse().unwrap(),
        }
    }

    fn regex() -> &'static regex::Regex {
        regex!(r"\((?P<grab>\d+)x(?P<repeat>\d+)\)")
    }

    fn find_all(s: &str) -> impl Iterator<Item = Marker> + '_ {
        Marker::regex()
            .captures_iter(s)
            .map(|cap| Marker::new(cap.get(0).unwrap(), &cap["grab"], &cap["repeat"]))
    }
}

type Markers = BTreeMap<usize, Marker>;

fn find_parent_markers(markers: &Markers, start: usize, end: usize) -> Vec<&Marker> {
    let mut parents = vec![];
    let mut parent = match markers.range(start..end).next() {
        Some((_, marker)) => marker,
        None => {
            return parents;
        }
    };
    parents.push(parent);
    while let Some((_, marker)) = markers.range(parent.end + parent.grab..end).next() {
        parents.push(marker);
        parent = marker;
    }
    parents
}

fn length_recursive_descent(
    markers: &Markers,
    multiplicity: usize,
    start: usize,
    end: usize,
) -> usize {
    let children = find_parent_markers(markers, start, end);
    if children.is_empty() {
        return (end - start) * multiplicity;
    }
    let last = children.last().unwrap();
    let trailing_len = (end - last.end - last.grab) * multiplicity;
    let intermarker_len = children
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(pre, post)| (post.start - pre.end - pre.grab) * multiplicity)
        .sum::<usize>();
    let children_len = children
        .into_iter()
        .map(|marker| {
            length_recursive_descent(
                markers,
                multiplicity * marker.repeat,
                marker.end,
                marker.end + marker.grab,
            )
        })
        .sum::<usize>();
    children_len + intermarker_len + trailing_len
}

pub struct OldSolver;

impl aoc::OldSolver<2016, 9> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let input = input.trim();
        let mut current_index = 0;
        let mut uncompressed_len = 0;

        for marker in Marker::find_all(input) {
            if marker.start < current_index {
                continue;
            }
            // Grab everything until the next marker's repeat sequence ends.
            // e.g. `(9x3)ABC(YxZ)DEF...`: grab `ABC(YxZ)D` repeating it thrice, leaving `EF...` for the next iteration.
            uncompressed_len += marker.start - current_index + marker.grab * marker.repeat;
            current_index = marker.end + marker.grab;
        }
        // Grab the rest of the string.
        uncompressed_len += input.len() - current_index;

        uncompressed_len
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let input = input.trim();
        let markers = Marker::find_all(input)
            .map(|marker| (marker.start, marker))
            .collect::<Markers>();
        let leading_len = match markers.values().next() {
            Some(marker) => marker.start,
            None => 0,
        };
        let children_len = length_recursive_descent(&markers, 1, 0, input.len());
        leading_len + children_len
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::aoc;
    use rstest::rstest;

    #[rstest]
    #[case("ADVENT".to_string(), 6)]
    #[case("A(1x5)BC".to_string(), 7)]
    #[case("(3x3)XYZ".to_string(), 9)]
    #[case("A(2x2)BCD(2x2)EFG".to_string(), 11)]
    #[case("(6x1)(1x3)A".to_string(), 6)]
    #[case("X(8x2)(3x3)ABCY".to_string(), 18)]
    fn test_solve_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(
            aoc::OldSolver::solve_part_one(&super::OldSolver, &input),
            expected
        )
    }

    #[rstest]
    #[case("(3x3)XYZ".to_string(), 9)]
    #[case("(1x14)IXYZ".to_string(), 17)]
    #[case("X(8x2)(3x3)ABCY".to_string(), 20)]
    #[case("(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string(), 241920)]
    #[case("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string(), 445)]
    #[case(
        "(27x12)(20x12)(13x14)(7x10)(1x12)A(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string(),
        242365
    )]
    #[case(
        "(3x3)XYZ(27x12)(20x12)(13x14)(7x10)(1x12)A(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string(),
        242374
    )]
    fn test_solve_part_two(#[case] input: String, #[case] expected: usize) {
        assert_eq!(
            aoc::OldSolver::solve_part_two(&super::OldSolver, &input),
            expected
        )
    }
}
