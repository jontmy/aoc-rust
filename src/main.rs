mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2021;
mod aoc2022;
mod aoc2024;
mod utils;

use chrono::{Datelike, Utc};
use clap::Parser;
use utils::v2::solver::Solver;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'd', long = "day")]
    day: Option<u32>,
    #[arg(short = 'y', long = "year")]
    year: Option<u32>,
    #[arg(short = 'r', long = "refetch")]
    refetch: bool,
    #[arg(short = 'i', long = "input")]
    input_file_extension: Option<String>,
}

fn main() {
    let args = Args::parse();
    let now = Utc::now();
    let day = args.day.unwrap_or(now.day());
    let month = now.month();
    let year = args
        .year
        .unwrap_or(now.year().try_into().expect("year should be u32"));

    if month != 12 && (args.day.is_none() || args.year.is_none()) {
        eprintln!("It's not December yet, please specify a day and year with -d and -y.");
        std::process::exit(1);
    }

    let _ = match year {
        2017 => match day {
            3 => aoc2017::day03::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            _ => {
                eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
                std::process::exit(1);
            }
        },
        2022 => match day {
            1 => aoc2022::day01::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            2 => aoc2022::day02::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            3 => aoc2022::day03::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            4 => aoc2022::day04::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            5 => aoc2022::day05::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            6 => aoc2022::day06::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            7 => aoc2022::day07::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            8 => aoc2022::day08::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            9 => aoc2022::day09::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            10 => aoc2022::day10::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            11 => aoc2022::day11::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            12 => aoc2022::day12::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            13 => aoc2022::day13::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            17 => aoc2022::day17::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            18 => aoc2022::day18::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            _ => {
                eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
                std::process::exit(1);
            }
        },
        2024 => match day {
            1 => aoc2024::day01::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            2 => aoc2024::day02::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            3 => aoc2024::day03::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            4 => aoc2024::day04::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            5 => aoc2024::day05::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            6 => aoc2024::day06::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            7 => aoc2024::day07::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            8 => aoc2024::day08::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            9 => aoc2024::day09::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            10 => aoc2024::day10::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            11 => aoc2024::day11::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            12 => aoc2024::day12::Solver.solve(args.refetch, args.input_file_extension.as_deref()),
            _ => {
                eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
            std::process::exit(1);
        }
    };
}
