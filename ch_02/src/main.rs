use std::{env, fs};

use colored::*;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    input_filename: String,
    output_filename: String,
}

impl Arguments {
    fn parse() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 4 {
            print_usage();
            eprintln!(
                "{} wrong number of arguments: expected 4, got {}.",
                "Error:".red().bold(),
                args.len()
            );
            std::process::exit(1);
        }
        Self {
            target: args[0].clone(),
            replacement: args[1].clone(),
            input_filename: args[2].clone(),
            output_filename: args[3].clone(),
        }
    }
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <input_filename> <output_filename>");
}

fn main() {
    let args = Arguments::parse();
    let input_data = match fs::read_to_string(&args.input_filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {}",
                "Error:".red().bold(),
                args.input_filename,
                e
            );
            std::process::exit(1);
        }
    };
    let replaced_data = match replace(&args.target, &args.replacement, &input_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };
    match fs::write(&args.output_filename, replaced_data) {
        Ok(_) => println!(
            "Successfully replaced text and wrote output to '{}'",
            args.output_filename
        ),
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {}",
                "Error:".red().bold(),
                args.output_filename,
                e
            );
            std::process::exit(1);
        }
    };
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}
