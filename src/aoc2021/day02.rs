use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    let re = Regex::new(r"(up|down|forward) (\d+)").unwrap();
    let (horizontal, depth) =
        re.captures_iter(input)
            .fold((0, 0), |(horizontal, depth), capture| {
                let delta = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                match capture.get(1).unwrap().as_str() {
                    "forward" => (horizontal + delta, depth),
                    "down" => (horizontal, depth + delta),
                    "up" => (horizontal, depth - delta),
                    _ => unreachable!(),
                }
            });

    horizontal * depth
}

pub fn solve_part_two(input: &String) -> i32 {
    let re = Regex::new(r"(up|down|forward) (\d+)").unwrap();
    let (horizontal, depth, _) =
        re.captures_iter(input)
            .fold((0, 0, 0), |(horizontal, depth, aim), capture| {
                let delta = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                match capture.get(1).unwrap().as_str() {
                    "forward" => (horizontal + delta, depth + aim * delta, aim),
                    "down" => (horizontal, depth, aim + delta),
                    "up" => (horizontal, depth, aim - delta),
                    _ => unreachable!(),
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
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "}.to_string(), 900)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
