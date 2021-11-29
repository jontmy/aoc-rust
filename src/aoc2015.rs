use crate::utils::io;
use crate::utils::io::read;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn run() {
    /*
    let input = read(1, 2015);
    io::write(1, 2015,
              day01::solve_part_one(&input),
              day01::solve_part_two(&input));

    let input = read(2, 2015);
    io::write(2, 2015,
              day02::solve_part_one(&input),
              day02::solve_part_two(&input));

    let input = read(3, 2015);
    io::write(3, 2015,
              day03::solve_part_one(&input),
              day03::solve_part_two(&input));

    let input = read(4, 2015);
    io::write(4, 2015,
              day04::solve_part_one(&input),
              day04::solve_part_two(&input));
    */
    let input = read(5, 2015);
    io::write(5, 2015,
              day05::solve_part_one(&input),
              day05::solve_part_two(&input));

    let input = read(6, 2015);
    io::write(6, 2015,
              day06::solve_part_one(&input),
              day06::solve_part_two(&input));
}