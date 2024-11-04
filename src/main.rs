mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2021;
mod aoc2022;
mod utils;

use chrono::{Datelike, Utc};
use clap::Parser;
use spinners::{Spinner, Spinners};
use utils::advent::{self, Solver};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'a', long = "all")]
    all: bool,
    #[arg(short = 'd', long = "day")]
    day: Option<u32>,
    #[arg(short = 'y', long = "year")]
    year: Option<u32>,
    #[arg(short = 'r', long = "refetch")]
    refetch: bool,
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

    println!(
        "\n{}",
        ansi_term::Style::new()
            .bold()
            .paint(format!("Advent of Code {year}, Day {day}"))
    );

    let mut spinner = Spinner::new(Spinners::Dots, "Fetching input...".into());
    match advent::fetch_input(day, year, args.refetch) {
        Ok((_, source)) => match source {
            advent::InputSource::File => {
                (&mut spinner).stop_and_persist("✔", "Input read from cache".into())
            }
            advent::InputSource::Web => {
                spinner.stop_and_persist("✔", "Input downloaded successfully".into())
            }
        },
        Err(e) => {
            spinner.stop_and_persist("✘", format!("Failed to download input: {e}"));
            std::process::exit(1);
        }
    }

    let mut spinner = Spinner::new(Spinners::Dots, "Solving part 1...".into());
    let tick = std::time::Instant::now();
    let answer = aoc2017::day03::Solver.solve_part_one("");
    let elapsed = tick.elapsed().as_secs_f64() * 1000.0;
    spinner.stop_and_persist(
        "✔",
        format!("Part 1 solved in {elapsed:.1}ms (answer: {answer})"),
    );

    let mut spinner = Spinner::new(Spinners::Dots, "Solving part 2...".into());
    let tick = std::time::Instant::now();
    let answer = aoc2017::day03::Solver.solve_part_two("");
    let elapsed = tick.elapsed().as_secs_f64() * 1000.0;
    spinner.stop_and_persist(
        "✔",
        format!("Part 2 solved in {elapsed:.1}ms (answer: {answer})"),
    );
}
