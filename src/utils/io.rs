use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use ansi_term::Style;

/// Reads to a string the puzzle input for a given day and year.
pub fn read(day: u32, year: u32) -> String {
    let path = format!("input/{}/{:02}.txt", year, day);
    fs::read_to_string(path).expect("Unable to read file")
}

/// Writes to a file the puzzle output for a given day and year, given the solutions to its two parts.
pub fn write<T: Display, U: Display>(day: u32, year: u32, part_one: T, part_two: U) {
    let path = format!("output/{}/{:02}.txt", year, day);
    let path = Path::new(&path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Unable to create output folder");
    let output = format!("{}\n{}", part_one, part_two);

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create the puzzle output for day {} year {}: {}", day, year, why),
        Ok(file) => file,
    };

    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("Couldn't write the puzzle output for day {} year {}: {}", day, year, why),
        _ => ()
    }

    println!("\n{}\nPart 1: {}\nPart 2: {}",
             Style::new().bold().paint(format!{"Advent of Code {}, Day {}", year, day}),
             Style::new().italic().paint(format!("{}", part_one)),
             Style::new().italic().paint(format!("{}", part_two))
    );
}