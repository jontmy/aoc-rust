use std::{
    error::Error,
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::Write,
    os,
    path::Path,
};

use ansi_term::Style;
use anyhow::{Context, Result};
use dotenv;

pub enum InputSource {
    File,
    Web,
}

pub fn fetch_input(day: u32, year: u32, refetch: bool) -> Result<(String, InputSource)> {
    dotenv::dotenv().ok();
    let session_token = dotenv::var("SESSION_TOKEN")
        .with_context(|| "environment variable SESSION_TOKEN should be set")?;

    let filename = format!("input/{year}/{day:02}.txt");
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let path = Path::new(&filename);

    if path.exists() && !refetch {
        let input = fs::read_to_string(path)?.trim().into();
        return Ok((input, InputSource::File));
    }

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .header(
            "User-Agent",
            "https://github.com/jontmy/aoc-rust/blob/master/src/utils/advent.py by jontmy",
        )
        .build()?;

    let input = client.execute(response).and_then(|res| res.text())?;
    if input.starts_with("Please don't repeatedly request this endpoint before it unlocks!") {
        eprintln!("Please wait until the puzzle unlocks before fetching the input.");
        std::process::exit(1);
    }

    let input = input.trim();
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, input)?;

    Ok((input.trim().into(), InputSource::Web))
}

pub trait Solver {
    fn solve_part_one(&self, input: &str) -> String;
    fn solve_part_two(&self, input: &str) -> String;
}

pub trait OldSolver<const YEAR: u32, const DAY: u32> {
    type Part1: Display;
    type Part2: Display;

    fn solve_part_one(&self, input: &str) -> Self::Part1;
    fn solve_part_two(&self, input: &str) -> Self::Part2;

    fn solve(&self) {
        // Use const generics to get the details of this puzzle from structs which implement this trait.
        let day = DAY;
        let year = YEAR;
        println!(
            "\n{}",
            Style::new()
                .bold()
                .paint(format!("Advent of Code {}, Day {}", year, day))
        );

        // Create the input file (and its parent directories).
        let input_path = format!("input/{}/{:02}.txt", year, day);
        let input_path = Path::new(&input_path);
        let input_dir = input_path
            .parent()
            .expect("Failed to get input parent directory for {year}");

        if !input_dir.exists() {
            println!("Input folder for {year} does not exist, creating it...");
        }
        fs::create_dir_all(input_dir).expect("Failed to create input folder for {year}");
        if !input_path.exists() {
            println!("Input file for day {day} of {year} does not exist, creating it...");
            File::create(&input_path).expect("Failed to create input file for day {day} of {year}");
        }

        // Read the input file.
        let input = fs::read_to_string(input_path).expect("Failed to read input file");

        // Create the output file (and its parent directories).
        let output_path = format!("output/{}/{:02}.txt", year, day);
        let output_path = Path::new(&output_path);
        let output_dir = output_path
            .parent()
            .expect("Failed to get output parent directory for year {year}");

        fs::create_dir_all(output_dir).expect("Failed to create output folder for {year}");
        File::create(&output_path).expect("Failed to create output file for day {day} of {year}");

        let mut file = OpenOptions::new()
            .append(true)
            .open(output_path)
            .expect("Failed to open output file for day {day} of {year}");

        // Solve part 1, writing to stdout and the output file.
        print!("Part 1: ");
        let part_one_solution = format!("{}", self.solve_part_one(&input));
        println!(
            "{}",
            Style::new()
                .italic()
                .paint(format!("{}", part_one_solution))
        );
        writeln!(file, "{part_one_solution}")
            .expect("Failed to write part 1 solution for day {day} of {year}");

        // Solve part 2, writing to stdout and the output file.
        print!("Part 2: ");
        let part_two_solution = format!("{}", self.solve_part_two(&input));
        println!(
            "{}",
            Style::new()
                .italic()
                .paint(format!("{}", part_two_solution))
        );
        writeln!(file, "{part_two_solution}")
            .expect("Failed to write part 2 solution for day {day} of {year}");
    }
}
