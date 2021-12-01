use crate::utils::io;
use crate::utils::io::read;

mod day01;

pub fn run() {
    let input = read(1, 2021);
    io::write(1, 2021,
              day01::solve_part_one(&input),
              day01::solve_part_two(&input));
}