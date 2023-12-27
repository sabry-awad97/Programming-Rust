use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("this code is running inside a catch_unwind closure");
        panic!("oh no, something went wrong!");
    });

    match result {
        Ok(()) => println!("the closure ran successfully"),
        Err(_) => println!("the closure panicked"),
    }

    println!("the program continues to run after the catch_unwind closure");
}
