use crate::utils::io;
use crate::utils::io::read;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn run() {
    /*
    let input = read(1, 2021);
    io::write(1, 2021,
              day01::solve_part_one(&input),
              day01::solve_part_two(&input));

    let input = read(2, 2021);
    io::write(2, 2021,
              day02::solve_part_one(&input),
              day02::solve_part_two(&input));

    let input = read(3, 2021);
    io::write(3, 2021,
              day03::solve_part_one(&input),
              day03::solve_part_two(&input));
    */

    let input = read(4, 2021);
    io::write(4, 2021,
              day04::solve_part_one(&input),
              day04::solve_part_two(&input));
}