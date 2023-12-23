use std::env;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            (m, n) = (n, m)
        }
        m %= n;
    }
    n
}

fn main() {
    // Parse command line arguments
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        // Attempt to parse each argument as a u64
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    // Handle empty input
    if numbers.is_empty() {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    // Calculate the GCD
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    // Print the result
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
