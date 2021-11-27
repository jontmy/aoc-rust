use crate::utils::io;
use crate::utils::io::read;

mod day01;
mod day02;

fn run_day(day: u32) {
    let input = read(day, 2015);
    io::write(day, 2015,
              day01::solve_part_one(&input),
              day01::solve_part_two(&input));
}

pub fn run_all_days() {
    for i in 1..2 {
        run_day(i);
    }
}