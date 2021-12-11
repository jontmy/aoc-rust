use crate::utils::io;
use crate::utils::io::read;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day11;
mod day10;

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

    let input = read(4, 2021);
    io::write(4, 2021,
              day04::solve_part_one(&input),
              day04::solve_part_two(&input));

    let input = read(5, 2021);
    io::write(5, 2021,
              day05::solve_part_one(&input),
              day05::solve_part_two(&input));

    let input = read(6, 2021);
    io::write(6, 2021,
              day06::solve_part_one(&input),
              day06::solve_part_two(&input));

    let input = read(7, 2021);
    io::write(7, 2021,
              day07::solve_part_one(&input),
              day07::solve_part_two(&input));

    let input = read(8, 2021);
    io::write(8, 2021,
              day08::solve_part_one(&input),
              day08::solve_part_two(&input));

    let input = read(9, 2021);
    io::write(9, 2021,
              day09::solve_part_one(&input),
              day09::solve_part_two(&input));
    */

    let input = read(10, 2021);
    io::write(10, 2021,
              day10::solve_part_one(&input),
              day10::solve_part_two(&input));
}