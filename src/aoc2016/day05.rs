use std::collections::HashMap;

use itertools::Itertools;

#[allow(dead_code)] // solver takes > 60s
pub fn solve_part_one(input: String) -> String {
    let input = input.trim();
    (0..)
        .into_iter()
        .map(|i| format!("{:x}", md5::compute(format!("{input}{i}"))))
        .filter(|hash| hash.starts_with("00000"))
        .take(8)
        .map(|hash| hash.chars().nth(5).unwrap())
        .collect()
}

#[allow(dead_code)] // solver takes > 60s
pub fn solve_part_two(input: String) -> String {
    let input = input.trim();
    let mut password = HashMap::new();

    // Find the character at each position.
    for i in (0..).into_iter() {
        let hash = format!("{:x}", md5::compute(format!("{input}{i}")));
        if !hash.starts_with("00000") {
            continue;
        }

        // Add only the first instance of a character in the correct position (between 0 and 7 inclusive).
        if let Some(pos) = hash.chars().nth(5).unwrap().to_digit(10) {
            if password.contains_key(&pos) || pos >= 8 {
                continue;
            }
            let val = hash.chars().nth(6).unwrap();
            password.insert(pos, val);
            println!("{pos}");
        }
        if password.len() == 8 {
            break;
        }
    }

    // Reconstruct the password in order.
    password.into_iter().sorted().map(|(_, val)| val).collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case("abc".to_string(), "18f47a30".to_string())]
    fn test_solve_part_one(#[case] input: String, #[case] expected: String) {
        assert_eq!(solve_part_one(input), expected)
    }

    #[rstest]
    #[case("abc".to_string(), "05ace8e3".to_string())]
    fn test_solve_part_two(#[case] input: String, #[case] expected: String) {
        assert_eq!(solve_part_two(input), expected);
    }
}
