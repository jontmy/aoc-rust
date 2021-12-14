use petgraph::prelude::{GraphMap, UnGraphMap};
use petgraph::Undirected;
use serde_scan::scan;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cave<'a> {
    Big(&'a str),
    Small(&'a str),
}

impl Cave<'_> {
    fn new(s: &str) -> Cave {
        match s.chars().all(|c: char| c.is_uppercase()) {
            true => Cave::Big(s),
            false => Cave::Small(s),
        }
    }

    fn is_named(&self, name: &str) -> bool {
        match &self {
            Cave::Big(s) => *s == name,
            Cave::Small(s) => *s == name,
        }
    }

    fn is_big(&self) -> bool {
        match &self {
            Cave::Big(s) => true,
            Cave::Small(s) => false,
        }
    }

    fn is_small(&self) -> bool {
        match &self {
            Cave::Big(s) => false,
            Cave::Small(s) => true,
        }
    }
}

fn parse_input(input: &String) -> GraphMap<Cave, (), Undirected> {
    let edges = input
        .lines()
        .map(|line| {
            let (a, b): (&str, &str) = scan!("{}-{}" <- line).unwrap();
            (Cave::new(a), Cave::new(b))
        })
        .collect::<Vec<_>>();

    UnGraphMap::<Cave, ()>::from_edges(edges)
}

pub fn solve_part_one(input: &String) -> i32 {
    let map = parse_input(input);
    let start = Cave::new("start");

    let mut paths = 0;
    let mut dfs = vec![(HashMap::from([(start, 1)]), start)];

    while !dfs.is_empty() {
        let (path, last) = dfs.pop().unwrap();
        if last.is_named("end") {
            paths += 1;
            continue;
        }
        map.neighbors(last)
            .filter(|cave| !path.contains_key(cave) || cave.is_big())
            .for_each(|cave| {
                let mut extended = path.clone();
                *extended.entry(cave).or_insert(0) += 1;
                dfs.push((extended, cave));
            })
    }
    paths
}

pub fn solve_part_two(input: &String) -> i32 {
    let map = parse_input(input);
    let start = Cave::new("start");

    let mut paths = 0;
    let mut dfs = vec![(HashMap::from([(start, 1)]), start)];

    while !dfs.is_empty() {
        let (path, last) = dfs.pop().unwrap();
        if last.is_named("end") {
            paths += 1;
            continue;
        }
        map.neighbors(last)
            .filter(|cave| {
                if cave.is_named("start") {
                    return false;
                }
                if cave.is_named("end") {
                    return true;
                }
                if cave.is_big() || !path.contains_key(cave) {
                    return true;
                }
                path.iter()
                    .filter(|(&k, &v)| k.is_small() && v >= 2)
                    .count()
                    == 0
            })
            .for_each(|cave| {
                let mut extended = path.clone();
                *extended.entry(cave).or_insert(0) += 1;
                dfs.push((extended, cave));
            })
    }
    paths
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "}.to_string(), 10)]
    #[case(indoc::indoc ! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "}.to_string(), 19)]
    #[case(indoc::indoc ! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "}.to_string(), 226)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "}.to_string(), 36)]
    #[case(indoc::indoc ! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "}.to_string(), 103)]
    #[case(indoc::indoc ! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "}.to_string(), 3509)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
