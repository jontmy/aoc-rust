pub fn solve_part_one(input: &String) -> i32 {
    input.lines().map(count_skipped_chars).sum()
}

fn count_skipped_chars(str: &str) -> i32 {
    let mut skipped = 2;
    let mut i = 1;
    let chars = str.chars().collect::<Vec<char>>();

    while i < str.len() - 1 {
        match chars[i] {
            '\\' => match chars[i + 1] {
                '\\' | '"' => {
                    i += 2;
                    skipped += 1;
                }
                'x' => {
                    i += 4;
                    skipped += 3;
                }
                _ => i += 1,
            },
            _ => i += 1,
        }
    }
    skipped
}

pub fn solve_part_two(input: &String) -> i32 {
    input
        .lines()
        .map(|s| s.chars().filter(|c| *c == '\\' || *c == '"').count() as i32 + 2)
        .sum()
}

// no tests written for 2015.08 because escaped strings are a pain to type out correctly
