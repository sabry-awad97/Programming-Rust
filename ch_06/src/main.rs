use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("Enter a number:");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(num) => {
                println!("You entered the number: {}", num);
                break;
            }
            Err(_) => {
                println!("Invalid input, please try again.");
                continue;
            }
        }
    }
}
