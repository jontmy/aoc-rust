use itertools::Itertools;

pub fn solve_part_one(input: String) -> String {
    input
        .lines()
        .flat_map(|line| line.chars().enumerate()) // take every character in the input and convert it into (column index, character)
        .into_group_map_by(|(col, _)| *col) // now group into a map every column index to a vector of (column index, character)
        .into_iter()
        .sorted_by_key(|(col, _)| *col) // we lose the column index order in the iterator during the grouping, so restore it
        .map(|(_, letters)| {
            let frequencies = letters // map each character in this column to its frequency
                .into_iter()
                .map(|(_, letter)| letter)
                .counts();
            let (letter, _) = frequencies // find the most common letter in this column and return it
                .into_iter()
                .max_by(|(_a, a_freq), (_b, b_freq)| a_freq.cmp(b_freq))
                .unwrap();
            letter
        })
        .collect() // concatenate the most common letter from each column
}

pub fn solve_part_two(input: String) -> String {
    input
        .lines()
        .flat_map(|line| line.chars().enumerate()) // take every character in the input and convert it into (column index, character)
        .into_group_map_by(|(col, _)| *col) // now group into a map every column index to a vector of (column index, character)
        .into_iter()
        .sorted_by_key(|(col, _)| *col) // we lose the column index order in the iterator during the grouping, so restore it
        .map(|(_, letters)| {
            let frequencies = letters // map each character in this column to its frequency
                .into_iter()
                .map(|(_, letter)| letter)
                .counts();
            let (letter, _) = frequencies // find the least common letter in this column and return it
                .into_iter()
                .min_by(|(_a, a_freq), (_b, b_freq)| a_freq.cmp(b_freq))
                .unwrap();
            letter
        })
        .collect() // concatenate the least common letter from each column
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(
        (
            indoc::indoc! {
                "
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar
    "
            }
        ).to_string(),
        "easter".to_string()
    )]
    fn test_solve_part_one(#[case] input: String, #[case] expected: String) {
        assert_eq!(solve_part_one(input), expected)
    }

    #[rstest]
    #[case(
        (
            indoc::indoc! {
                "
        eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar
    "
            }
        ).to_string(),
        "advent".to_string()
    )]
    fn test_solve_part_two(#[case] input: String, #[case] expected: String) {
        assert_eq!(solve_part_two(input), expected);
    }
}
