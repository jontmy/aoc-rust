use std::iter;

use itertools::Itertools;

pub fn solve_part_one(input: &String) -> String {
    let initial_password = input.lines().next().unwrap().to_string();
    next_valid_password(initial_password)
}

pub fn solve_part_two(input: &String) -> String {
    next_valid_password(increment_password(&solve_part_one(input)))
}

fn next_valid_password(pwd: String) -> String {
    iter::successors(Some(pwd), |s| Some(increment_password(s)))
        .find(|pwd| is_valid_password(pwd))
        .unwrap()
}

fn is_valid_password(pwd: &String) -> bool {
    let bytes = pwd.bytes().collect_vec();

    // 1. Passwords must include one increasing straight of at least three letters without skipping letters.
    let fulfils_increasing_requirement = pwd
        .bytes()
        .enumerate()
        .take(pwd.len() - 2)
        .any(|(i, c)| bytes[i + 1] == c + 1 && bytes[i + 2] == c + 2);

    if !fulfils_increasing_requirement {
        return false;
    }

    // 2. Passwords may not contain the letters i, o, or l.
    let fulfils_exclusion_requirement = pwd.chars().all(|c| c != 'i' && c != 'o' && c != 'l');

    if !fulfils_exclusion_requirement {
        return false;
    }

    // 3. Passwords must contain at least two different, non-overlapping pairs of letters.
    let fulfils_pair_requirements = pwd
        .bytes()
        .enumerate()
        .filter(|(i, c)| *i > 0 && bytes[i - 1] == *c)
        .map(|(i, c)| c)
        .dedup()
        .count()
        >= 2;

    fulfils_increasing_requirement && fulfils_exclusion_requirement && fulfils_pair_requirements
}

// Potential optimization: skip all 'i', 'l' and 'o's.
fn increment_password(pwd: &String) -> String {
    let mut chars = pwd.bytes().collect_vec();
    for c in chars.iter_mut().rev() {
        *c += 1; // increment the last letter, carrying as needed
        if *c > 0x7a {
            // wrap an incremented 'z' back around to 'a'
            *c = 0x61;
        } else {
            break;
        }
    }
    String::from_utf8(chars).unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{is_valid_password, solve_part_one};

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    #[case("ghjaabcc", true)]
    fn test_is_valid_password(#[case] input: String, #[case] expected: bool) {
        assert_eq!(expected, is_valid_password(&input))
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_part_one(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, solve_part_one(&input))
    }
}
