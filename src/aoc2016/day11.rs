use std::{ collections::{ HashSet, VecDeque }, str::FromStr, iter, hash::Hash };

use itertools::Itertools;
use once_cell_regex::regex;

use crate::utils::advent;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Item {
    Generator(String),
    Microchip(String),
    Nothing,
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self {
            Item::Generator(e) => format!("G({e})"),
            Item::Microchip(e) => format!("M({e})"),
            Item::Nothing => "".to_string(),
        }
    }
}

impl Item {
    fn is_nothing(&self) -> bool {
        match self {
            Item::Nothing => true,
            _ => false,
        }
    }

    fn is_generator(&self) -> bool {
        match self {
            Item::Generator(_) => true,
            _ => false,
        }
    }

    fn is_microchip(&self) -> bool {
        match self {
            Item::Microchip(_) => true,
            _ => false,
        }
    }

    fn element(&self) -> &str {
        match self {
            Item::Generator(e) => e,
            Item::Microchip(e) => e,
            Item::Nothing => panic!("Cannot get element of Item::Nothing"),
        }
    }

    fn will_fry(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Generator(a), Item::Microchip(b)) => a != b,
            (Item::Microchip(a), Item::Generator(b)) => a != b,
            (Item::Generator(_), Item::Generator(_)) => false,
            (Item::Microchip(_), Item::Microchip(_)) => false,
            (Item::Nothing, _) | (_, Item::Nothing) => false,
        }
    }
}

#[derive(Debug)]
enum Elevator {
    Descend,
    Ascend,
}

#[derive(Debug, Clone)]
struct Floor {
    id: usize,
    items: HashSet<Item>,
}

impl ToString for Floor {
    fn to_string(&self) -> String {
        let items = self.items
            .iter()
            .map(|item| item.to_string())
            .sorted()
            .join(", ");
        format!("{}: {items}", self.id)
    }
}

impl Floor {
    fn new(floor: usize, s: &str) -> Self {
        let generator_regex = regex!(r"(?P<generator>\w+) generator");
        let microchip_regex = regex!(r"(?P<microchip>\w+)-compatible microchip");
        let generators = generator_regex
            .captures_iter(s)
            .map(|cap| Item::Generator(cap["generator"].to_string()));
        let microchips = microchip_regex
            .captures_iter(s)
            .map(|cap| Item::Microchip(cap["microchip"].to_string()));
        let items = generators.chain(microchips).collect();
        Self { id: floor, items }
    }

    fn remove(&mut self, to_remove: &[Item]) {
        assert!(to_remove.into_iter().all(|item| (item.is_nothing() || self.items.contains(item))));
        for item in to_remove {
            self.items.remove(item);
        }
    }

    fn add(&mut self, to_add: &[Item]) {
        assert!(to_add.into_iter().all(|item| !self.items.contains(item)));
        for item in to_add {
            if item.is_nothing() {
                continue;
            }
            self.items.insert(item.clone());
        }
    }

    fn will_fry(&self) -> bool {
        let generators = self.items
            .iter()
            .filter(|i| i.is_generator())
            .map(|i| i.element())
            .collect::<HashSet<_>>();

        // If a chip is ever left in the same area as another RTG, and it's not connected to its own RTG, the chip will be fried.
        self.items
            .iter()
            .filter(|i| i.is_microchip())
            .map(|i| i.element())
            .any(|chip| !generators.contains(chip) && generators.iter().any(|gen| *gen != chip))
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[derive(Debug, Clone)]
struct State {
    depth: usize, // BFS depth, not related to the elevator
    elevator_floor: usize,
    floors: Vec<Floor>,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let floors = s
            .lines()
            .enumerate()
            .map(|(i, s)| Floor::new(i, s))
            .collect_vec();
        Ok(Self { depth: 0, elevator_floor: 0, floors })
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        let floors = self.floors.iter()
            .map(|floor| floor.to_string())
            .join("\n");
        format!("elevator @ floor {}\n{floors}", self.elevator_floor)
    }
}

impl State {
    fn with_elevator_floor(floor: usize, s: &str) -> Self {
        let mut res = s.parse::<Self>().unwrap();
        res.elevator_floor = floor;
        res
    }

    fn next_states(&self) -> impl Iterator<Item = Self> + '_ {
        // The elevator can ascend or descend at most one floor, carrying at most 2 items each time.
        let floor = self.floors[self.elevator_floor].clone();
        floor.items
            .into_iter()
            .chain(iter::once(Item::Nothing))
            .combinations(2)
            .flat_map(|items|
                vec![
                    self.next_state(&items, Elevator::Ascend),
                    self.next_state(&items, Elevator::Descend)
                ]
            )
            .filter_map(|state| state) // remove invalid states
    }

    fn next_state(&self, elevator_items: &[Item], action: Elevator) -> Option<Self> {
        assert_eq!(elevator_items.len(), 2);
        // Reject the state transition if the elevator items will fry each other.
        if elevator_items[0].will_fry(&elevator_items[1]) {
            return None;
        }
        // Reject the state transition if the elevator will go out of bounds.
        let next_floor = match action {
            Elevator::Ascend => if self.elevator_floor == self.floors.len() - 1 {
                return None;
            } else {
                self.elevator_floor + 1
            }
            Elevator::Descend => if self.elevator_floor == 0 {
                return None;
            } else {
                self.elevator_floor - 1
            }
        };
        // Compute the next state, rejecting floors with any item that will fry each other.
        let mut next_state = self.clone();
        next_state.depth += 1;
        next_state.elevator_floor = next_floor;
        next_state.floors[self.elevator_floor].remove(elevator_items);
        next_state.floors[next_floor].add(elevator_items);
        // println!("\n\n{:?}\n{:?}\n\n{:?}", action, elevator_items, next_state);
        match next_state.is_valid_state() {
            true => Some(next_state),
            false => None,
        }
    }

    fn is_valid_state(&self) -> bool {
        self.floors
            .iter()
            .all(|floor| !floor.will_fry())
    }

    fn is_goal_state(&self) -> bool {
        self.floors
            .iter()
            .take(self.floors.len() - 1)
            .all(|floor| floor.is_empty())
    }
}

pub struct Solver;

impl advent::Solver<2016, 11> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let initial_state = State::with_elevator_floor(0, input);
        let mut evaluated = HashSet::new();
        let mut stack = VecDeque::new();
        // println!("{}", initial_state.to_string());
        // println!("{:?}", initial_state.next_states().collect_vec());
        evaluated.insert(initial_state.to_string());
        stack.push_back(initial_state);
        loop {
            let eval_state = stack.pop_front().unwrap();
            evaluated.insert(eval_state.to_string());
            println!("{:?}", eval_state.depth);
            if eval_state.is_goal_state() {
                return eval_state.depth;
            }
            for next_state in eval_state.next_states() {
                if evaluated.contains(&next_state.to_string()) {
                    continue;
                }
                stack.push_back(next_state);
            }
        }
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        0
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::utils::advent;

    use super::State;

    #[rstest]
    #[case(
        (
            indoc::indoc! {
                "
                The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
                The second floor contains a hydrogen generator.
                The third floor contains a lithium generator.
                The fourth floor contains nothing relevant.
                "
            }
        ).to_string(),
        11
    )]
    fn test_solve_part_one(#[case] input: String, #[case] expected: usize) {
        assert_eq!(advent::Solver::solve_part_one(&super::Solver, &input), expected)
    }

    #[rstest]
    #[case(
        (
            indoc::indoc! {
                "
                The first floor contains nothing.
                The second floor contains nothing.
                The third floor contains nothing.
                The fourth floor ccontains a hydrogen-compatible microchip and a lithium-compatible microchip.
                "
            }
        ).to_string(),
        true
    )]
    #[case(
        (
            indoc::indoc! {
                "
                The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
                The second floor contains a hydrogen generator.
                The third floor contains a lithium generator.
                The fourth floor contains nothing relevant.
                "
            }
        ).to_string(),
        false
    )]
    fn test_is_goal_state(#[case] input: String, #[case] expected: bool) {
        assert_eq!(input.parse::<State>().unwrap().is_goal_state(), expected)
    }
}