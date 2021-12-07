use itertools::Itertools;

pub fn solve_part_one(input: &String) -> usize {
    input.lines()
        .map(|l| l.parse::<i32>().unwrap())
        .powerset()
        .filter(|containers| containers.into_iter().sum::<i32>() == 150)
        .count()
}

pub fn solve_part_two(input: &String) -> usize {
    let combinations = input.lines()
        .map(|l| l.parse::<i32>().unwrap())
        .powerset()
        .filter(|containers| containers.into_iter().sum::<i32>() == 150)
        .collect_vec();

    let min_containers = combinations.iter()
        .min_by(|lhs, rhs| lhs.len().cmp(&rhs.len()))
        .unwrap().len();

    combinations.into_iter()
        .filter(|containers| containers.len() == min_containers)
        .count()
}
