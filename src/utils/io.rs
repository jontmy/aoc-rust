use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Reads to a string the puzzle input for a given day and year.
pub fn read(day: u32, year: u32) -> String {
    let path = format!("input/{}/{:02}.txt", year, day);
    fs::read_to_string(path).expect("Unable to read file")
}

/// Writes to a file the puzzle output for a given day and year, given the solutions to its two parts.
pub fn write<T: Display, U: Display>(day: u32, year: u32, first_part: T, second_part: U) {
    let path = format!("output/{}/{:02}.txt", year, day);
    let path = Path::new(&path);
    let output = format!("{}\n{}", first_part, second_part);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create the puzzle output for day {} year {}: {}", day, year, why),
        Ok(file) => file,
    };

    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("Couldn't write the puzzle output for day {} year {}: {}", day, year, why),
        _ => ()
    }
}