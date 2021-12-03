use itertools::Itertools;

pub fn solve_part_one(input: &String) -> i32 {
    let (gamma, epsilon): (Vec<char>, Vec<char>) = frequencies(input).iter()
        .map(|(zeroes, ones)| if zeroes > ones { ('0', '1') } else { ('1', '0') })
        .unzip();

    let gamma = i32::from_str_radix(&*gamma.iter().collect::<String>(), 2).unwrap();
    let epsilon = i32::from_str_radix(&*epsilon.iter().collect::<String>(), 2).unwrap();

    gamma * epsilon
}

// Converts a vector of binary strings into a vector of frequency('0', '1') tuples at each index.
fn frequencies(bins: &String) -> Vec<(usize, usize)> {
    let index_char_map = bins.lines()
        .flat_map(|bin| bin.chars().enumerate())
        .into_group_map();

    (0..index_char_map.len())
        .map(|i| {
            let (zeroes, ones): (Vec<char>, Vec<char>) = index_char_map.get(&i)
                .unwrap()
                .iter()
                .partition(|&&c| c == '0');
            (zeroes.len(), ones.len())
        })
        .collect_vec()
}

pub fn solve_part_two(input: &String) -> i32 {
    let lines = input.lines()
        .map(|s| s.to_string())
        .collect_vec();
    let len = lines[0].len();

    let initial = (
        lines.iter().collect_vec(),
        lines.iter().collect_vec()
    );

    let (o2, co2) = (0..len).into_iter()
        .fold(initial, |(o2, co2), i| {
            let (o2_bit, co2_bit) = bit_criteria(&o2, &co2, i);

            let o2 = match o2.len() {
                1 => o2,
                _ => o2.iter()
                    .filter(|&&bin| bin[i..=i] == o2_bit)
                    .map(|s| *s)
                    .collect_vec()
            };

            let co2 = match co2.len() {
                1 => co2,
                _ => co2.iter()
                    .filter(|&&bin| bin[i..=i] == co2_bit)
                    .map(|s| *s)
                    .collect_vec()
            };

            (o2, co2)
        });

    let o2_rating = i32::from_str_radix(o2[0], 2).unwrap();
    let co2_rating = i32::from_str_radix(co2[0], 2).unwrap();

    o2_rating * co2_rating
}

fn bit_criteria(o2: &Vec<&String>, co2: &Vec<&String>, index: usize) -> (String, String) {
    let (zeroes, ones): (Vec<char>, Vec<char>) = o2.iter()
        .map(|bin| bin.chars()
            .nth(index)
            .unwrap())
        .partition(|&c| c == '0');

    let most_common = if ones.len() >= zeroes.len() { '1' } else { '0' };

    let (zeroes, ones): (Vec<char>, Vec<char>) = co2.iter()
        .map(|bin| bin.chars()
            .nth(index)
            .unwrap())
        .partition(|&c| c == '0');

    let least_common = if zeroes.len() <= ones.len() { '0' } else { '1' };

    (most_common.to_string(), least_common.to_string())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 150)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"

    "}.to_string(), 900)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}