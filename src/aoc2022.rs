use crate::utils::advent::Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub fn run() {
    day01::Solver.solve();
    day02::Solver.solve();
    day03::Solver.solve();
    day04::Solver.solve();
    day05::Solver.solve();
    day06::Solver.solve();
}