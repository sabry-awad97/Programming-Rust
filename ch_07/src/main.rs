use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    let result: Result<i32, &str> = Ok(42);
    let and_then_result = result.and_then(|val| Ok(val * 2));

    let read_integer = |file: &mut File| {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).and_then(|_| {
            buffer.trim().parse::<i32>().map_err(|err| {
                println!("Failed to parse integer: {}", err);
                Error::new(ErrorKind::InvalidData, "Invalid integer")
            })
        })
    };

    let mut file = File::open("data.txt").unwrap();
    let integer = read_integer(&mut file);
}
