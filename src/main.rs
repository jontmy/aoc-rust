mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2021;
mod aoc2022;
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

    let solver = match year {
        2017 => match day {
            3 => Some(aoc2017::day03::Solver),
            _ => None,
        },
        _ => None,
    };

    if let Some(solver) = solver {
        if let Err(e) = solver.solve(args.refetch) {
            eprintln!("✘ Failed to solve: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("✘ No solver available for day {day} of Advent of Code {year}");
        std::process::exit(1);
    }
}
