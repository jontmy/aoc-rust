use json::JsonValue;
use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    let re: Regex = Regex::new(r"(-?\d+)").unwrap();
    re.captures_iter(input)
        .map(|capture| capture.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .sum()
}

pub fn solve_part_two(input: &String) -> i32 {
    count(&json::parse(input).unwrap())
}

fn count(data: &JsonValue) -> i32 {
    return if data.is_number() {
        data.as_i32().unwrap()
    } else if data.is_array() {
        data.members()
            .map(|x| count(&x))
            .sum()
    } else if data.is_object() {
        if data.entries().any(|(_, v)| v.is_string() && v == "red") {
            0
        } else {
            data.entries()
                .map(|(_, v)| count(&v))
                .sum()
        }
    } else {
        0
    };
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case("{\"a\":2,\"b\":4}", 6)]
    #[case("[[[3]]]", 3)]
    #[case("{\"a\":{\"b\":4},\"c\":-1}", 3)]
    #[case("{\"a\":[-1,1]}", 0)]
    #[case("[-1,{\"a\":1}]", 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case("[1,{\"c\":\"red\",\"b\":2},3]", 4)]
    #[case("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0)]
    #[case("[1,\"red\",5]", 6)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}