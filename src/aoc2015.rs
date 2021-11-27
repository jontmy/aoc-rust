use crate::utils::io;
use crate::utils::io::read;

mod day01;
mod day02;

pub fn run() {
    let input = read(1, 2015);
    io::write(1, 2015,
              day01::solve_part_one(&input),
              day01::solve_part_two(&input));

    let input = read(2, 2015);
    io::write(2, 2015,
              day02::solve_part_one(&input),
              day02::solve_part_two(&input));
}