use once_cell_regex::regex;

use crate::utils::advent;

#[derive(Debug, Clone)]
struct Marker { // in the format (<grab>x<repeat>)
    start: usize, // index in the input string of the opening parentheses `(`
    end: usize, // index of the next character after the closing parentheses `)`
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

pub struct Solver;

impl advent::Solver<2016, 9> for Solver {
    type Part1 = usize;
    type Part2 = i32;

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

    fn solve_part_two(&self, _input: &str) -> Self::Part2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("ADVENT".to_string(), 6)]
    #[case("A(1x5)BC".to_string(), 7)]
    #[case("(3x3)XYZ".to_string(), 9)]
    #[case("A(2x2)BCD(2x2)EFG".to_string(), 11)]
    #[case("(6x1)(1x3)A".to_string(), 6)]
    #[case("X(8x2)(3x3)ABCY".to_string(), 18)]
    fn test_address_supports_tls(#[case] input: String, #[case] expected: usize) {
        assert_eq!(crate::utils::advent::Solver::solve_part_one(&super::Solver, &input), expected)
    }
}