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

pub trait Solver {
    fn solve_part_one(&self, input: &str) -> String;
    fn solve_part_two(&self, input: &str) -> String;
}
