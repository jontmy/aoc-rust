use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    Regex::new(r"(up|down|forward) (\d+)").unwrap()
        .captures_iter(input)
        .for_each(|capture| {
            let delta = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            match capture.get(1).unwrap().as_str() {
                "forward" => horizontal += delta,
                "down" => depth += delta,
                "up" => depth -= delta,
                _ => unreachable!()
            }
        });

    horizontal * depth
}

pub fn solve_part_two(input: &String) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    Regex::new(r"(up|down|forward) (\d+)").unwrap()
        .captures_iter(input)
        .for_each(|capture| {
            let delta = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
            match capture.get(1).unwrap().as_str() {
                "forward" => {
                    horizontal += delta;
                    depth += aim * delta;
                }
                "down" => aim += delta,
                "up" => aim -= delta,
                _ => unreachable!()
            }
        });

    horizontal * depth
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "}.to_string(), 150)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "}.to_string(), 5)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}