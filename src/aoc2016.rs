use crate::utils::io;

mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

pub fn run() {
    io::write(3, 2016,
        day03::solve_part_one(io::read(3, 2016)),
        day03::solve_part_two(io::read(3, 2016))
    );
    io::write(4, 2016,
        day04::solve_part_one(io::read(4, 2016)),
        day04::solve_part_two(io::read(4, 2016))
    );
    // day 5 skipped - solution takes > 60s to derive
    // io::write(5, 2016,
    //     day05::solve_part_one(io::read(5, 2016)),
    //     day05::solve_part_two(io::read(5, 2016))
    // );
    io::write(6, 2016,
        day06::solve_part_one(io::read(6, 2016)),
        day06::solve_part_two(io::read(6, 2016))
    );
    io::write(7, 2016,
        day07::solve_part_one(io::read(7, 2016)),
        day07::solve_part_two(io::read(7, 2016))
    );
}