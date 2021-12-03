
use itertools::Itertools;

pub fn solve_part_one(input: &String) -> i32 {
    let bins = input.lines()
        .map(|s| s.to_string())
        .collect_vec();

    let (gamma, epsilon) = gamma_epsilon(&bins.iter().collect_vec());
    let gamma = isize::from_str_radix(gamma.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2).unwrap();

    (gamma * epsilon).try_into().unwrap()
}

// Converts a vector of binary strings into a vector of frequency('0', '1') tuples at each index.
fn frequencies(bins: &Vec<&String>) -> Vec<(i32, i32)> {
    let mut freqs = vec!((0, 0); bins[0].len());
    for bin in bins {
        for (i, c) in bin.chars().enumerate() {
            match c {
                '0' => freqs[i].0 += 1,
                '1' => freqs[i].1 += 1,
                _ => panic!(),
            }
        }
    };
    freqs
}

fn gamma_epsilon(bins: &Vec<&String>) -> (String, String) {
    let (gamma, epsilon): (Vec<_>, Vec<_>) = frequencies(bins).iter()
        .map(|(zeroes, ones)| {
            if zeroes > ones {
                ('0', '1')
            } else {
                ('1', '0')
            }
        })
        .unzip();

    (gamma.iter().collect::<String>(), epsilon.iter().collect::<String>())
}

pub fn solve_part_two(input: &String) -> i32 {
    let lines = input.lines()
        .map(|s| s.to_string())
        .collect_vec();
    let len = lines[0].len();

    let mut o2 = lines.iter().collect_vec();
    for i in 0..len {
        let (gamma, _) = gamma_epsilon(&o2);
        o2 = o2.iter()
            .filter(|&bin| bin[i..=i] == gamma[i..=i])
            .map(|s| *s)
            .collect_vec();
        dbg!(&o2);
        if o2.len() <= 1 { break }
    }

    let mut co2 = lines.iter().collect_vec();
    for i in 0..len {
        let (_, epsilon) = gamma_epsilon(&co2);
        co2 = co2.iter()
            .filter(|&bin| bin[i..=i] == epsilon[i..=i])
            .map(|s| *s)
            .collect_vec();
        dbg!(epsilon);
        dbg!(&co2);
        if co2.len() <= 1 { break }
    }

    let o2_rating = isize::from_str_radix(o2[0], 2).unwrap();
    let co2_rating = isize::from_str_radix(co2[0], 2).unwrap();

    (o2_rating * co2_rating).try_into().unwrap()
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