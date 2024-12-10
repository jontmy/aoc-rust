use std::collections::VecDeque;

use itertools::Itertools;

use crate::utils::v2::solver;

#[derive(Debug)]
struct Fragment {
    files: VecDeque<File>,
    free_block_size: u8,
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    block_size: u8,
    has_moved: bool,
}

#[derive(Debug)]
enum Span {
    File(File),
    EmptySpace(u8),
}

pub struct Solver;

impl Solver {
    fn parse_input(&self, input: &str) -> Vec<Fragment> {
        let input = input
            .chars()
            .map(|c| c as u8 - '0' as u8)
            .chain([0])
            .collect_vec();

        assert!(input.len() % 2 == 0);
        let fragments = input
            .chunks(2)
            .enumerate()
            .map(|(i, chunk)| Fragment {
                files: VecDeque::from(vec![File {
                    id: i,
                    block_size: chunk[0],
                    has_moved: false,
                }]),
                free_block_size: chunk[1],
            })
            .collect_vec();

        fragments
    }
}

impl solver::Solver<2024, 9> for Solver {
    type Part1 = usize;
    type Part2 = usize;

    fn solve_part_one(&self, input: &str) -> Self::Part1 {
        let mut fragments = self.parse_input(input);
        let mut target_fragment_idx = 0;
        let mut data_fragment_idx = fragments.len() - 1;

        while data_fragment_idx > target_fragment_idx {
            let target_fragment = &fragments[target_fragment_idx];
            let data_fragment = &fragments[data_fragment_idx];

            if target_fragment.free_block_size == 0 {
                target_fragment_idx += 1;
                continue;
            }
            assert!(target_fragment.free_block_size > 0);
            assert!(data_fragment.files.len() == 1);

            let data_file = &data_fragment.files[0];
            let data_file_id = data_file.id;
            let blocks_to_move = target_fragment.free_block_size.min(data_file.block_size);

            let target_fragment = &mut fragments[target_fragment_idx];
            target_fragment.free_block_size -= blocks_to_move;
            target_fragment.files.push_back(File {
                id: data_file_id,
                block_size: blocks_to_move,
                has_moved: true,
            });

            let data_fragment = &mut fragments[data_fragment_idx];
            data_fragment.files[0].block_size -= blocks_to_move;
            data_fragment.free_block_size += blocks_to_move;
            if data_fragment.files[0].block_size == 0 {
                data_fragment_idx -= 1;
                data_fragment.files.clear();
            }
        }

        let mut checksum = 0;
        let mut block_id = 0;
        for fragment in fragments {
            for file in &fragment.files {
                for _ in 0..file.block_size {
                    checksum += block_id * file.id;
                    block_id += 1;
                }
            }
        }
        checksum
    }

    fn solve_part_two(&self, input: &str) -> Self::Part2 {
        let fragments = self.parse_input(input);
        let mut fs = fragments
            .into_iter()
            .flat_map(move |frag| {
                [
                    Span::File(frag.files[0].clone()),
                    Span::EmptySpace(frag.free_block_size),
                ]
            })
            .collect_vec();

        let mut i = fs.len() - 1;
        loop {
            let file = match &fs[i] {
                Span::File(file) if !file.has_moved => file,
                _ => {
                    i -= 1;
                    continue;
                }
            }
            .clone();
            for j in 0..i {
                if let Span::EmptySpace(block_size) = &fs[j] {
                    if block_size < &file.block_size {
                        continue;
                    }
                    fs[j] = Span::EmptySpace(block_size - file.block_size);
                    fs[i] = Span::EmptySpace(file.block_size);
                    fs.insert(
                        j,
                        Span::File(File {
                            has_moved: true,
                            ..file
                        }),
                    );
                    break;
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        let mut checksum = 0;
        let mut block_id = 0;
        for span in fs {
            match span {
                Span::File(file) => {
                    for _ in 0..file.block_size {
                        checksum += block_id * file.id;
                        block_id += 1;
                    }
                }
                Span::EmptySpace(block_size) => {
                    block_id += block_size as usize;
                }
            }
        }
        checksum
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::v2::solver::Solver;
    use rstest::rstest;

    #[rstest]
    #[case("2333133121414131402", 1928)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        let solver = super::Solver;
        let result = solver.solve_part_one(input);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("2333133121414131402", 2858)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        let solver = super::Solver;
        let result = solver.solve_part_two(input);
        assert_eq!(result, expected);
    }
}
