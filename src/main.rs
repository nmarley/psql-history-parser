use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Simple program to parse psql_history files and convert octal codes to text
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the .psql_history file
    filename: String,
}

/// Function to convert octal escape sequences to their character equivalents.
fn convert_octal_to_text(octal_str: &str) -> String {
    let re = Regex::new(r"\\([0-7]{3})").unwrap();
    re.replace_all(octal_str, |caps: &regex::Captures| {
        let octal_value = u8::from_str_radix(&caps[1], 8).unwrap();
        (octal_value as char).to_string()
    })
    .to_string()
}

/// Function to read the psql_history file and convert octal codes.
fn parse_psql_history<P: AsRef<Path>>(filename: P) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let converted_line = convert_octal_to_text(&line);
        println!("{}", converted_line);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    parse_psql_history(args.filename)
}
