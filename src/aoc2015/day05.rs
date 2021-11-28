use std::ops::{Not};

pub fn solve_part_one(input: &String) -> i32 {
    input.lines()
        .filter(is_nice_string_one)
        .count() as i32
}

fn is_nice_string_one(s: &&str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    // 1. Contains at least three vowels (aeiou only).
    let fulfils_vowels_condition = s.chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false
        })
        .count() >= 3;

    // 2. Contains at least one letter that appears twice in a row.
    let fulfils_contiguous_condition = s.chars()
        .enumerate()
        .any(|(i, c)| {
            let prev = (i as i32 - 1 > 0) && (chars[i - 1] == c);
            let next = (i + 1 < s.len()) && (chars[i + 1] == c);
            prev || next
        });

    // 3. Does not contain the strings ab, cd, pq, or xy,
    //    even if they are part of one of the other requirements.
    let fulfills_exclusion_condition = ["ab", "cd", "pq", "xy"].iter()
        .any(|ex| s.contains(ex))
        .not();

    // A nice string is one with all of the above properties.
    fulfils_vowels_condition && fulfils_contiguous_condition && fulfills_exclusion_condition
}

pub fn solve_part_two(input: &String) -> i32 {
    input.lines()
        .filter(is_nice_string_two)
        .count() as i32
}

fn is_nice_string_two(s: &&str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    // 1. Contains a pair of any two letters that appears at least twice in the string without overlapping.
    let pairs = s.chars()
        .zip(s.chars().skip(1)) // form the pairs
        .map(|(a, b)| format!("{}{}", a, b))
        .collect::<Vec<String>>();

    let fulfils_pair_condition = pairs.iter()
        .enumerate()
        .any(|(i, c)| {
            pairs.iter()
                .enumerate()
                .any(|(j, d)| c == d && (i as i32 - j as i32).abs() > 1)
        });

    // 2. Contains at least one letter which repeats with exactly one letter between them.
    let fulfils_repetition_condition = chars.iter()
        .enumerate()
        .any(|(i, c)| i < s.len() - 2 && chars[i] == chars[i + 2]);

    // A nice string is one with all of the above properties.
    fulfils_pair_condition && fulfils_repetition_condition
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};
    use rstest::rstest;

    #[rstest]
    #[case("aaa", 1)]
    #[case("ugknbfddgicrmopn", 1)]
    #[case("jchzalrnumimnmhp", 0)]
    #[case("haegwjzuvuyypxyu", 0)]
    #[case("dvszwmarrgswjxmb", 0)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("aaa", 0)]
    #[case("xyxy", 1)]
    #[case("qjhvhtzxzqqjkmpb", 1)]
    #[case("uurcxstgmygtbstg", 0)]
    #[case("ieodomkazucvgmuy", 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}