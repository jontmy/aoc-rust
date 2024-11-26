use itertools::Itertools;

use crate::utils::aoc;

pub struct OldSolver;

impl OldSolver {
    fn cmp(i: usize, a_str: &str, b_str: &str) -> usize {
        let mut ad = 0;
        let mut bd = 0;
        let mut at = vec![];
        let mut bt = vec![];

        for x in a_str.chars() {
            match x {
                '[' => {
                    ad += 1;
                }
                ']' => {
                    ad -= 1;
                }
                ',' => {
                    continue;
                }
                a => at.push((a.to_digit(10).expect(format!("{}", x).as_str()), ad)),
            }
        }

        for y in b_str.chars() {
            match y {
                '[' => {
                    bd += 1;
                }
                ']' => {
                    bd -= 1;
                }
                ',' => {
                    continue;
                }
                b => bt.push((b.to_digit(10).unwrap(), bd)),
            }
        }

        let mut b_ptr = 0;
        let mut a_ptr = 0;

        loop {
            if bt.len() == 0 {
                return 0;
            }

            if a_ptr >= at.len() || b_ptr >= bt.len() {
                break;
            }

            if let Some((a, _)) = at.get(a_ptr) {
                if *a == 10 {
                    a_ptr += 1;
                    continue;
                }
            }

            if let Some((b, _)) = bt.get(b_ptr) {
                if *b == 10 {
                    b_ptr += 1;
                    continue;
                }
            }

            let (_, ad) = at[a_ptr];
            let (_, bd) = bt[b_ptr];

            let a_list = at[a_ptr..]
                .into_iter()
                .take_while(|(x, xd)| *xd == ad && *x != 10)
                .copied()
                .collect_vec();

            let b_list = bt[b_ptr..]
                .into_iter()
                .take_while(|(x, xd)| *xd == bd && *x != 10)
                .copied()
                .collect_vec();

            println!("{:?}", a_list);
            println!("{:?}", b_list);

            a_ptr += a_list.len();
            b_ptr += b_list.len();

            for something in a_list.into_iter().zip_longest(b_list) {
                match something {
                    itertools::EitherOrBoth::Both((a, _), (b, _)) => {
                        // println!("{:?}", (a, b));
                        if a > b {
                            return 0;
                        } else if a < b {
                            return i;
                        }
                    }
                    itertools::EitherOrBoth::Left(_) => {
                        return 0;
                    }
                    itertools::EitherOrBoth::Right(_) => {
                        return i;
                    }
                }
            }
        }
        return i;
    }
}

impl aoc::OldSolver<2022, 13> for OldSolver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut ans = 0;
        let pairs = input.trim().split("\n\n");

        for (i, inn) in pairs.enumerate() {
            let j = inn.split("\n").collect_tuple();
            let (a, b) = j.unwrap();
            println!("{}", OldSolver::cmp(i + 1, a, b) != 0);
            ans += OldSolver::cmp(i + 1, a, b);
        }

        ans
    }

    fn solve_part_two(&self, _input: &str) -> Self::Part2 {
        0
    }
}
