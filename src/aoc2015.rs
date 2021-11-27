use crate::utils::io;
use crate::utils::io::read;

pub mod day01;

pub fn run() {

    // There's probably a better way to do this without duplicating code, but it works for now.
    let day01 = read(01, 2015);
    io::write(1, 2015,
              day01::solve_part_one(&day01),
              day01::solve_part_two(&day01));
}