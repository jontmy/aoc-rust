use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

pub fn solve_part_one(input: &String) -> i32 {
    let relations = parse_input(input);
    let people = relations.keys()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<HashSet<&str>>();

    // What is the total change in happiness for the optimal seating arrangement of
    // the actual guest list?
    best_happiness(people, relations)
}

pub fn solve_part_two(input: &String) -> i32 {
    let mut relations = parse_input(input);
    let mut people = relations.keys()
        .flat_map(|(a, b)| [*a, *b])
        .collect::<HashSet<&str>>();

    // Give all happiness relationships that involve you a score of 0.
    people.iter()
        .flat_map(|&person| [(person, "Dummy"), ("Dummy", person)])
        .for_each(|k| {
            relations.insert(k, 0);
        });

    // Add yourself to the list.
    people.insert("Dummy");

    // What is the total change in happiness for the optimal seating arrangement that
    // actually includes yourself?
    best_happiness(people, relations)
}

fn parse_input(input: &String) -> HashMap<(&str, &str), i32> {
    Regex::new(r"(?m)(\w+) would (gain|lose) (\d+) .*? to (\w+)").unwrap()
        .captures_iter(input.as_str())
        .map(|capture| {
            let person = capture.get(1).unwrap().as_str();
            let beside = capture.get(4).unwrap().as_str();
            let sign = match capture.get(2).unwrap().as_str() {
                "gain" => 1,
                "lose" => -1,
                _ => panic!()
            };
            let change_in_happiness = sign * capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
            ((person, beside), change_in_happiness)
        })
        .collect::<HashMap<(&str, &str), i32>>()
}

fn permutation_happiness(perm: Vec<&&str>, relations: &HashMap<(&str, &str), i32>) -> i32 {
    perm.iter()
        .circular_tuple_windows::<(_, _, _)>()
        .map(|(l, c, r)| {
            relations.get(&(**c, **l)).unwrap() + relations.get(&(**c, **r)).unwrap()
        })
        .sum()
}

fn best_happiness(people: HashSet<&str>, relations: HashMap<(&str, &str), i32>) -> i32 {
    people.iter().permutations(people.len())
        .map(|perm| permutation_happiness(perm, &relations))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "}.to_string(), 330)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case("str", 0)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}