use itertools::Itertools;

static BOARD_SIZE: usize = 5;

pub fn solve_part_one(input: &String) -> i32 {
    let (numbers, boards) = parse_input(input);

    // Pick the board that can be won in the least number of numbers drawn.
    let (winning_board, winning_draws) = boards.iter()
        .map(|board| (board, board.numbers_to_win(&numbers)))
        .filter(|(_, draws)| draws.is_some())
        .map(|(board, draws)| (board, draws.unwrap()))
        .sorted_by(|(_, this), (_, that)| this.len().cmp(&that.len()))
        .next()
        .unwrap();

    winning_board.checksum(winning_draws)
}

pub fn solve_part_two(input: &String) -> i32 {
    let (numbers, boards) = parse_input(input);

    // Pick the board that can be won in the least number of numbers drawn.
    let (winning_board, winning_draws) = boards.iter()
        .map(|board| (board, board.numbers_to_win(&numbers)))
        .filter(|(_, draws)| draws.is_some())
        .map(|(board, draws)| (board, draws.unwrap()))
        .sorted_by(|(_, this), (_, that)| this.len().cmp(&that.len()))
        .last()
        .unwrap();

    winning_board.checksum(winning_draws)
}

struct Board {
    grid: Vec<i32>,
}

impl Board {
    fn parse(chunk: &[&str]) -> Board {
        Board {
            grid: chunk.iter()
                .flat_map(|&row| row.split_whitespace())
                .map(|tile| tile.parse().unwrap())
                .collect::<Vec<i32>>()
        }
    }

    // Returns true if the board is won given a slice of numbers are drawn, or false otherwise.
    fn is_won(&self, drawn: &[i32]) -> bool {
        let matches = self.grid.iter()
            .map(|tile| drawn.contains(tile))
            .collect_vec();

        let any_row_won = *&matches.chunks(BOARD_SIZE)
            .map(|row| row.iter().all(|tile| *tile))
            .any(|row| row);

        let any_col_won = (0..BOARD_SIZE).into_iter()
            .map(|offset| {
                matches.iter()
                    .skip(offset)
                    .step_by(BOARD_SIZE)
                    .all(|tile| *tile)
            })
            .any(|col| col);

        any_row_won || any_col_won
    }

    // Returns the slice of numbers that need to be drawn in order for the board to be won,
    // if and only if the board can be won.
    fn numbers_to_win<'a>(&self, drawn: &'a [i32]) -> Option<&'a [i32]> {
        (0..drawn.len()).into_iter()
            .map(|i| &drawn[0..=i])
            .skip_while(|drawn| !self.is_won(drawn))
            .next()
    }

    // Returns the sum of all unmarked numbers that are unmarked (not drawn),
    // multiplied by the last number drawn.
    fn checksum(&self, drawn: &[i32]) -> i32 {
        self.grid.iter()
            .filter(|tile| !drawn.contains(tile))
            .sum::<i32>() * drawn.last().unwrap()
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
        .map(|chunk| Board::parse(chunk))
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