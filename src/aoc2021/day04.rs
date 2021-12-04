use itertools::Itertools;

pub fn solve_part_one(input: &String) -> i32 {
    let (numbers, boards) = parse_input(input);

    for i in 0..numbers.len() {
        let drawn = &numbers[0..=i];
        let sum = boards.iter()
            .filter(|board| board.is_won(drawn))
            .map(|board| board.unmarked(drawn))
            .next();

        match sum {
            Some(sum) => {
                return sum.iter().sum::<i32>() * numbers[i]
            },
            None => continue
        }
    }
    unreachable!()
}

pub fn solve_part_two(input: &String) -> i32 {
    let (numbers, boards) = parse_input(input);

    let (i, last) = boards.iter()
        .map(|board| {
            for i in 0..numbers.len() {
                let drawn = &numbers[0..=i];
                if board.is_won(drawn) {
                    return (i as i32, board);
                }
            }
            (-1, board)
        })
        .sorted_by(|(i, _), (j, _)| i.cmp(j))
        .last()
        .unwrap();

    return last.unmarked(&numbers[0..(i as usize + 1)]).iter().sum::<i32>() * numbers[i as usize]

}

struct Board {
    grid: Vec<Vec<i32>>
}

impl Board {
    fn parse(chunk: &[&str]) -> Board {
        let grid = chunk.iter()
            .map(|&row | {
                row.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<_>>();

        Board {
            grid
        }
    }

    fn is_won(&self, drawn: &[i32]) -> bool {
        let matches = self.grid.iter()
            .map(|row| {
                row.iter()
                    .map(|i| drawn.contains(i))
                    .collect_vec()
            })
            .collect_vec();


        for row in &matches {
            if row.iter().all(|i| *i) {
                return true;
            }
        }
        for col in 0..5 {
            if (0..5).all(|row| matches[row][col]) {
                return true;
            }
        }
        false
    }

    fn unmarked(&self, drawn: &[i32]) -> Vec<i32> {
        self.grid.iter()
            .flat_map(|row| row)
            .filter(|i| !drawn.contains(i))
            .map(|i| *i)
            .collect_vec()
    }
}

fn parse_input(input: &String) -> (Vec<i32>, Vec<Board>) {
    let numbers = input.lines().next().unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect_vec();

    let boards = input.lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .collect_vec()
        .chunks(5)
        .map(|ch|Board::parse(ch))
        .collect_vec();

    (numbers, boards)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_one, solve_part_two};

    #[rstest]
    #[case(indoc::indoc ! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "}.to_string(), 4512)]
    fn test_part_one(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_one(&input))
    }

    #[rstest]
    #[case(indoc::indoc ! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "}.to_string(), 1924)]
    fn test_part_two(#[case] input: String, #[case] expected: i32) {
        assert_eq!(expected, solve_part_two(&input))
    }
}