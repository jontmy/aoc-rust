use crate::utils::io;

mod day03;
mod day04;

pub fn run() {
    io::write(3, 2016,
        day03::solve_part_one(io::read(3, 2016)),
        day03::solve_part_two(io::read(3, 2016))
    );
    io::write(4, 2016,
        day04::solve_part_one(io::read(4, 2016)),
        day04::solve_part_two(io::read(4, 2016))
    );
}