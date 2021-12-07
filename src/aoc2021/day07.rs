use itertools::Itertools;

pub fn solve_part_one(input: &String) -> i32 {
    let crabs = input.split(',').into_iter()
        .map(|i| i.parse::<i32>().unwrap())
        .collect_vec();

    let (min, max) = crabs.iter()
        .minmax()
        .into_option().unwrap();

    (*min..=*max).into_iter()
        .map(|i| {
            crabs.iter()
                .map(|crab| (*crab - i).abs())
                .sum()
        })
        .min().unwrap()
}

pub fn solve_part_two(input: &String) -> i32 {
    let crabs = input.split(',').into_iter()
        .map(|i| i.parse::<i32>().unwrap())
        .collect_vec();

    let (min, max) = crabs.iter()
        .minmax()
        .into_option().unwrap();

    (*min..=*max).into_iter()
        .map(|i| {
            crabs.iter()
                .map(|crab| (*crab - i).abs())
                .map(|n| n * (n + 1) / 2)
                .sum()
        })
        .min().unwrap()
}


#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case("16,1,2,0,4,2,7,1,2,14", 37)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("16,1,2,0,4,2,7,1,2,14", 168)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}