use std::ops::Not;

pub fn solve_part_one(input: &String) -> i32 {
    input.lines()
        .filter(is_nice_string)
        .count() as i32
}

fn is_nice_string(s: &&str) -> bool {
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
    //    even if they are part of one of the other requirements
    let fulfills_exclusion_condition = ["ab", "cd", "pq", "xy"].iter()
        .any(|ex| s.contains(ex))
        .not();

    // A nice string is one with all of the above properties.
    fulfils_vowels_condition && fulfils_contiguous_condition && fulfills_exclusion_condition
}

pub fn solve_part_two(input: &String) -> i32 {
    0
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
    #[case("aaa", 1)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}