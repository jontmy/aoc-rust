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
mod day10;
mod day12;
mod day11;
mod day13;

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

    let input = read(5, 2015);
    io::write(5, 2015,
              day05::solve_part_one(&input),
              day05::solve_part_two(&input));

    let input = read(6, 2015);
    io::write(6, 2015,
              day06::solve_part_one(&input),
              day06::solve_part_two(&input));

    let input = read(7, 2015);
    io::write(7, 2015,
              day07::solve_part_one(&input),
              day07::solve_part_two(&input));

    let input = read(8, 2015);
    io::write(8, 2015,
              day08::solve_part_one(&input),
              day08::solve_part_two(&input));

    let input = read(9, 2015);
    io::write(9, 2015,
              day09::solve_part_one(&input),
              day09::solve_part_two(&input));

    let input = read(10, 2015);
    io::write(10, 2015,
              day10::solve_part_one(&input),
              day10::solve_part_two(&input));

    let input = read(11, 2015);
    io::write(11, 2015,
              day11::solve_part_one(&input),
              day11::solve_part_two(&input));
    */

    let input = read(12, 2015);
    io::write(12, 2015,
              day12::solve_part_one(&input),
              day12::solve_part_two(&input));

    let input = read(13, 2015);
    io::write(13, 2015,
              day13::solve_part_one(&input),
              day13::solve_part_two(&input));
}