use crate::utils::io;

mod day03;

pub fn run() {
    io::write(3, 2016,
        day03::solve_part_one(io::read(3, 2016)),
        day03::solve_part_two(io::read(3, 2016))
    );
}