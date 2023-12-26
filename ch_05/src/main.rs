fn find_first_even(numbers: &[i32]) -> Option<&i32> {
    numbers.iter().find(|&number| number % 2 == 0)
}

fn main() {
    let numbers = [1, 3, 5, 7, 9];
    let result = find_first_even(&numbers);

    match result {
        Some(n) => println!("The first even number is {}", n),
        None => println!("There are no even numbers in the list"),
    }
}
