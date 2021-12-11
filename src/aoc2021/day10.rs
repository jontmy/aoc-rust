use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use self::PairedDelimiter::{Close, Open};

pub fn solve_part_one(input: &String) -> i64 {
    input.lines()
         .map(self::corrupt)
         .sum()
}

pub fn solve_part_two(input: &String) -> i64 {
    let scores = input.lines()
                      .filter(|l| corrupt(l) == 0)
                      .map(self::complete)
                      .sorted()
                      .collect_vec();

    scores[scores.len() / 2] // always odd
}

#[derive(PartialEq, Eq)]
enum PairedDelimiter {
    Open(char),
    Close(char),
}

impl PairedDelimiter {
    fn wrap(c: char) -> Self {
        match c {
            '(' => Self::Open('('),
            '[' => Self::Open('['),
            '{' => Self::Open('{'),
            '<' => Self::Open('<'),
            ')' => Self::Close(')'),
            ']' => Self::Close(']'),
            '}' => Self::Close('}'),
            '>' => Self::Close('>'),
            _ => panic!(),
        }
    }

    fn invert(&self) -> Self {
        match self {
            Self::Open('(') => Self::Close(')'),
            Self::Open('[') => Self::Close(']'),
            Self::Open('{') => Self::Close('}'),
            Self::Open('<') => Self::Close('>'),
            Self::Close(')') => Self::Open('('),
            Self::Close(']') => Self::Open('['),
            Self::Close('}') => Self::Open('{'),
            Self::Close('>') => Self::Open('<'),
            _ => panic!(),
        }
    }

    fn corrupted_score(self) -> i64 {
        match self {
            Self::Close(')') => 3,
            Self::Close(']') => 57,
            Self::Close('}') => 1197,
            Self::Close('>') => 25137,
            _ => panic!(),
        }
    }

    fn completed_score(self) -> i64 {
        match self {
            Self::Close(')') => 1,
            Self::Close(']') => 2,
            Self::Close('}') => 3,
            Self::Close('>') => 4,
            _ => panic!(),
        }
    }
}

fn corrupt(s: &str) -> i64 {
    let (_, illegal) = s.chars().into_iter()
                        .map(PairedDelimiter::wrap)
                        .fold_while((Vec::new(), None), |(mut stack, illegal), current| {
                            let previous = stack.last();
                            let expected = previous.map(PairedDelimiter::invert);

                            match (previous, &current) {
                                (Some(previous), Open(_)) => {
                                    stack.push(current);
                                    Continue((stack, None))
                                }
                                (Some(previous), Close(_)) => {
                                    stack.pop();
                                    if expected.unwrap() == current {
                                        Continue((stack, None))
                                    } else {
                                        Done((stack, Some(current)))
                                    }
                                }
                                (None, _) => {
                                    stack.push(current);
                                    Continue((stack, None))
                                }
                            }
                        }).into_inner();

    illegal.map(PairedDelimiter::corrupted_score).unwrap_or(0)
}

fn complete(s: &str) -> i64 {
    s.chars().into_iter()
     .map(PairedDelimiter::wrap)
     .fold(Vec::new(), |mut stack, current| {
         match (stack.last(), &current) {
             (Some(previous), Open(_)) => {
                 stack.push(current);
             }
             (Some(previous), Close(_)) => {
                 stack.pop();
             }
             (None, _) => stack.push(current),
         };
         stack
     }).iter().rev()
     .map(PairedDelimiter::invert)
     .fold(0, |mut score, delimiter| {
         score *= 5;
         score += delimiter.completed_score();
         score
     })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "}.to_string(), 26397)]
    fn test_part_one(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "}.to_string(), 288957)]
    fn test_part_two(#[case] input: String, #[case] expected: i64) {
        assert_eq!(expected, solve_part_two(&input))
    }
}