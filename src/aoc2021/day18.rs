use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;
use num_integer::Integer;

pub fn solve_part_one(input: &String) -> u32 {
    let snailfishes = input
        .lines()
        .map(|line| {
            let mut sn = line.parse::<SnailfishNumbers>().unwrap();
            sn.reduce();
            sn
        })
        .reduce(|acc, sn| acc + sn)
        .unwrap();

    snailfishes.magnitude()
}

pub fn solve_part_two(input: &String) -> u32 {
    input
        .lines()
        .permutations(2)
        .map(|p| p.into_iter().join("\n"))
        .map(|s| solve_part_one(&s))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct SnailfishNumber {
    value: u32,
    depth: i32,
}

impl SnailfishNumber {
    fn new(value: u32, depth: i32) -> SnailfishNumber {
        SnailfishNumber { value, depth }
    }

    fn add(&mut self, value: u32) {
        self.value += value;
    }
}

struct SnailfishNumbers {
    numbers: Vec<SnailfishNumber>,
}

impl FromStr for SnailfishNumbers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut depth = 0;
        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => continue,
                _ if c.is_digit(10) => {
                    let value = c.to_digit(10).unwrap();
                    let number = SnailfishNumber::new(value, depth);
                    numbers.push(number);
                }
                _ => panic!(),
            }
        }
        Ok(SnailfishNumbers { numbers })
    }
}

impl ToString for SnailfishNumbers {
    fn to_string(&self) -> String {
        self.numbers.iter().map(|sn| sn.value).join(" ")
    }
}

impl Add for SnailfishNumbers {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.numbers
            .iter_mut()
            .chain(rhs.numbers.iter_mut())
            .for_each(|sn| sn.depth += 1);

        let mut snailfishes = SnailfishNumbers {
            numbers: self
                .numbers
                .into_iter()
                .chain(rhs.numbers.into_iter())
                .collect_vec(),
        };
        snailfishes.reduce();
        snailfishes
    }
}

impl SnailfishNumbers {
    fn explode(&mut self) {
        if let Some(i) = self.numbers.iter().position(|sn| sn.depth > 4) {
            let d = self.numbers.get(i).unwrap().depth;
            let lv = self.numbers.get(i).unwrap().value;
            let rv = self.numbers.get(i + 1).expect(&*self.to_string()).value;
            if let Some(l) = i.checked_sub(1).map(|i| self.numbers.get_mut(i)).flatten() {
                l.add(lv);
            }
            if let Some(r) = i.checked_add(2).map(|i| self.numbers.get_mut(i)).flatten() {
                r.add(rv);
            }
            self.numbers.remove(i + 1);
            *self.numbers.get_mut(i).unwrap() = SnailfishNumber::new(0, d - 1);
        }
    }

    fn split(&mut self) {
        if let Some(i) = self.numbers.iter().position(|sn| sn.value > 9) {
            let sn = self.numbers.get(i).unwrap();
            let lv = sn.value.div_floor(&2);
            let rv = sn.value.div_ceil(2);
            let l = SnailfishNumber::new(lv, sn.depth + 1);
            let r = SnailfishNumber::new(rv, sn.depth + 1);
            self.numbers.insert(i, l);
            *self.numbers.get_mut(i + 1).unwrap() = r;
        }
    }

    fn reduce(&mut self) {
        while self.numbers.iter().any(|sn| sn.depth > 4 || sn.value > 9) {
            while self.numbers.iter().any(|sn| sn.depth > 4) {
                self.explode();
            }
            self.split();
        }
    }

    fn magnitude(mut self) -> u32 {
        while self.numbers.len() > 1 {
            let max_depth = self.numbers.iter().map(|sn| sn.depth).max().unwrap();

            let max_index = self
                .numbers
                .iter()
                .position(|sn| sn.depth == max_depth)
                .unwrap();

            let lv = self.numbers[max_index].value;
            let rv = self.numbers[max_index + 1].value;
            let coalesced = SnailfishNumber::new(3 * lv + 2 * rv, max_depth - 1);

            *self.numbers.get_mut(max_index).unwrap() = coalesced;
            self.numbers.remove(max_index + 1);
        }
        self.numbers[0].value
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        [[1,2],[[3,4],5]]
    "}.to_string(), 143)]
    #[case(indoc::indoc ! {"
        [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
    "}.to_string(), 1384)]
    #[case(indoc::indoc ! {"
        [[[[1,1],[2,2]],[3,3]],[4,4]]
    "}.to_string(), 445)]
    #[case(indoc::indoc ! {"
        [[[[3,0],[5,3]],[4,4]],[5,5]]
    "}.to_string(), 791)]
    #[case(indoc::indoc ! {"
        [[[[5,0],[7,4]],[5,5]],[6,6]]
    "}.to_string(), 1137)]
    #[case(indoc::indoc ! {"
        [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]
    "}.to_string(), 3488)]
    #[case(indoc::indoc ! {"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "}.to_string(), 4140)]
    fn test_part_one(#[case] input: String, #[case] expected: u32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "}.to_string(), 3993)]
    fn test_part_two(#[case] input: String, #[case] expected: u32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}
