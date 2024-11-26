use spinners::{Spinner, Spinners};
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

pub trait Solver<const YEAR: u32, const DAY: u32> {
    fn solve_part_one(&self, input: &str) -> String;
    fn solve_part_two(&self, input: &str) -> String;

    fn fetch_input(&self, refetch: bool) -> Result<(String, InputSource)> {
        dotenv::dotenv().ok();
        let session_token = dotenv::var("SESSION_TOKEN")
            .with_context(|| "environment variable SESSION_TOKEN should be set")?;

        let filename = format!("input/{YEAR}/{DAY:02}.txt");
        let url = format!("https://adventofcode.com/{YEAR}/DAY/{DAY}/input");
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
                "https://github.com/jontmy/aoc-rust/blob/master/src/utils/v2/aoc.rs by jontmy",
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

    fn solve_part(&self, part: u8, input: &str) -> Result<()> {
        let mut spinner = Spinner::new(Spinners::Dots, format!("Solving part {}...", part));
        let tick = std::time::Instant::now();

        let answer = match part {
            1 => self.solve_part_one(input),
            2 => self.solve_part_two(input),
            _ => unreachable!(),
        };

        let elapsed = tick.elapsed().as_secs_f64() * 1000.0;
        spinner.stop_and_persist(
            "✔",
            format!(
                "Part {} solved in {:.1}ms (answer: {})",
                part, elapsed, answer
            ),
        );
        Ok(())
    }

    fn solve(&self, refetch: bool) -> Result<()> {
        let mut spinner = Spinner::new(Spinners::Dots, "Fetching input...".into());
        match self.fetch_input(refetch) {
            Ok((input, source)) => {
                match source {
                    InputSource::File => {
                        (&mut spinner).stop_and_persist("✔", "Input read from cache".into())
                    }
                    InputSource::Web => {
                        spinner.stop_and_persist("✔", "Input downloaded successfully".into())
                    }
                }
                self.solve_part(1, &input)?;
                self.solve_part(2, &input)?;
                Ok(())
            }
            Err(e) => {
                spinner.stop_and_persist("✖", format!("Failed to fetch input: {}", e));
                Err(e.into())
            }
        }
    }
}
