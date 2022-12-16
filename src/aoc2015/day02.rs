pub fn solve_part_one(input: &String) -> i32 {
    input.lines().map(wrapping_paper_area).sum()
}

fn wrapping_paper_area(dimensions: &str) -> i32 {
    let mut lwh: Vec<i32> = dimensions
        .split('x')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    lwh.sort();

    if let [l, w, h] = lwh[0..3] {
        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
        let slack_area = l * w;
        return surface_area + slack_area;
    }
    panic!()
}

pub fn solve_part_two(input: &String) -> i32 {
    input.lines().map(ribbon_area).sum()
}

fn ribbon_area(dimensions: &str) -> i32 {
    let mut lwh: Vec<i32> = dimensions
        .split('x')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    lwh.sort();

    if let [l, w, h] = lwh[0..3] {
        let wrapping_area = l + l + w + w;
        let bow_area = l * w * h;
        return wrapping_area + bow_area;
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use crate::aoc2015::day02::{solve_part_one, solve_part_two};
    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", 58)]
    #[case("1x1x10", 43)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("2x3x4", 34)]
    #[case("1x1x10", 14)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
