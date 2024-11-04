mod aoc2015;
mod aoc2016;
mod aoc2021;
mod aoc2022;
mod utils;

use chrono::{Datelike, Utc};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'a', long = "all")]
    all: bool,
    #[arg(short = 'd', long = "day")]
    day: Option<u32>,
    #[arg(short = 'y', long = "year")]
    year: Option<u32>,
}

fn main() {
    let args = Args::parse();
    if args.all {
        aoc2015::run();
        aoc2016::run();
        aoc2021::run();
        aoc2022::run();
        return;
    }

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
    utils::advent::fetch_input(day, year);
}
