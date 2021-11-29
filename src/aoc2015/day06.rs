use regex::{Regex};

pub fn solve_part_one(input: &String) -> i32 {
    let mut grid = Grid::new();

    let re = Regex::new(r"(?m)^(turn off|turn on|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let captures = re.captures_iter(input)
        .for_each(|capture| {
            grid.act(
                capture.get(1).map_or("", |m| m.as_str()),
                capture.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
                capture.get(3).map_or(0, |m| m.as_str().parse().unwrap()),
                capture.get(4).map_or(0, |m| m.as_str().parse().unwrap()),
                capture.get(5).map_or(0, |m| m.as_str().parse().unwrap()),
            )
        });

    grid.count_lit()
}

// The 1000 x 1000 grid has only 1 million entries, so a 2D array is fine, though not technically ideal.
struct Grid {
    grid: [[u8; 1000]; 1000]
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: [[0; 1000]; 1000]
        }
    }

    fn act(&mut self, action: &str, x_start: usize, y_start: usize, x_end: usize, y_end: usize) {
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                self.grid[x][y] = match action {
                    "turn on" => 1,
                    "turn off" => 0,
                    "toggle" => if self.grid[x][y] == 0 {1} else {0},
                    _ => panic!()
                };
            }
        }
    }

    fn count_lit(&self) -> i32 {
        self.grid.iter().map(|row| {
            row.iter()
                .filter(|&e| *e == 1)
                .count() as i32
        })
        .sum()
    }
}

pub fn solve_part_two(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};
    use rstest::rstest;

    #[rstest]
    #[case("turn on 0,0 through 999,999", 1_000_000)]
    #[case("toggle 0,0 through 999,0", 1000)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("^v", 3)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}