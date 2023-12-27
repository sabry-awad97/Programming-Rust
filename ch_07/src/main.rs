use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_and_sum(filename: &str) -> GenericResult<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let num = line.trim().parse::<i32>()?;
        sum += num;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = read_and_sum("numbers.txt");

    match result {
        Ok(sum) => println!("Sum of numbers in file: {}", sum),
        Err(e) => {
            if let Some(parse_err) = e.downcast_ref::<ParseIntError>() {
                println!("Error: failed to parse integer: {}", parse_err);
            } else if let Some(io_err) = e.downcast_ref::<io::Error>() {
                println!("Error: I/O error occurred: {}", io_err);
            } else {
                println!("Unknown error occurred: {}", e);
            }
        }
    }

    Ok(())
}
