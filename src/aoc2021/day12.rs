use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

use bimap::BiHashMap;
use itertools::Itertools;
use petgraph::graph::UnGraph;
use petgraph::visit::Bfs;

pub fn solve_part_one(input: &String) -> i32 {
    let vertices = input
        .lines()
        .flat_map(|l| l.split("-"))
        .unique()
        .collect_vec();

    let vertices_refs: Vec<&str> = vertices.iter().map(|x| *x).collect();

    let edges = input
        .lines()
        .map(|l| {
            l.split("-")
                .map(|s| s.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .collect_vec();

    let edges_refs: Vec<(&str, &str)> = edges
        .iter()
        .map(|&(ref x, ref y)| (x.as_str(), y.as_str()))
        .collect();

    let mut graph = UnGraph::<&str, ()>::new_undirected();
    let vertices_refs = vertices_refs
        .into_iter()
        .map(|v| (v, graph.add_node(v)))
        .collect::<BiHashMap<_, _>>();

    for (a, b) in edges_refs {
        graph.update_edge(
            *vertices_refs.get_by_left(a).unwrap(),
            *vertices_refs.get_by_left(b).unwrap(),
            (),
        );
    }

    let source = *vertices_refs.get_by_left("start").unwrap();
    let mut queue = VecDeque::from([vec![source]]);
    let mut paths = 0;

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let last = path.last().unwrap();

        if *vertices_refs.get_by_right(last).unwrap() == "end" {
            paths += 1;
            continue;
        }

        graph
            .neighbors(*last)
            .filter(|neighbor| {
                !path.contains(neighbor)
                    || vertices_refs
                        .get_by_right(neighbor)
                        .unwrap()
                        .chars()
                        .all(|c: char| c.is_uppercase())
            })
            .for_each(|neighbor| {
                let mut extended = path.clone();
                extended.push(neighbor);
                queue.push_back(extended);
            })
    }

    paths
}

pub fn solve_part_two(input: &String) -> i32 {
    let vertices = input
        .lines()
        .flat_map(|l| l.split("-"))
        .unique()
        .collect_vec();

    let vertices_refs: Vec<&str> = vertices.iter().map(|x| *x).collect();

    let edges = input
        .lines()
        .map(|l| {
            l.split("-")
                .map(|s| s.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .collect_vec();

    let edges_refs: Vec<(&str, &str)> = edges
        .iter()
        .map(|&(ref x, ref y)| (x.as_str(), y.as_str()))
        .collect();

    let mut graph = UnGraph::<&str, ()>::new_undirected();
    let vertices_refs = vertices_refs
        .into_iter()
        .map(|v| (v, graph.add_node(v)))
        .collect::<BiHashMap<_, _>>();

    for (a, b) in edges_refs {
        graph.update_edge(
            *vertices_refs.get_by_left(a).unwrap(),
            *vertices_refs.get_by_left(b).unwrap(),
            (),
        );
    }

    let source = *vertices_refs.get_by_left("start").unwrap();
    let mut queue = VecDeque::from([vec![source]]);
    let mut paths = 0;

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let last = path.last().unwrap();

        if *vertices_refs.get_by_right(last).unwrap() == "end" {
            paths += 1;
            continue;
        }

        graph
            .neighbors(*last)
            .filter(|neighbor| {
                let frequencies = path.iter().counts();
                let cave = *vertices_refs.get_by_right(neighbor).unwrap();
                let visits = *frequencies.get(neighbor).unwrap_or(&0);
                let is_big = cave.chars().all(|c: char| c.is_uppercase());

                // Big caves can be visited any number of times.
                if is_big {
                    return true;
                }

                let small_visits = frequencies
                    .keys()
                    .filter(|id| {
                        vertices_refs
                            .get_by_right(id)
                            .unwrap()
                            .chars()
                            .all(|c: char| c.is_lowercase())
                    })
                    .map(|id| *frequencies.get(id).unwrap_or(&0))
                    .filter(|visits| *visits >= 2)
                    .count();

                // A single small cave can be visited at most twice.
                return if !is_big && visits == 0 {
                    true
                } else {
                    small_visits == 0 && cave != "start"
                }
            })
            .for_each(|neighbor| {
                let mut extended = path.clone();
                extended.push(neighbor);
                queue.push_back(extended);
            })
    }

    paths
}

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| {
            l.split("-")
                .map(|s| s.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
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
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
