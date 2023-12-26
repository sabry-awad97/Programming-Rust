fn print_slice(slice: &[i32]) {
    for item in slice {
        println!("{}", item);
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // creates a slice of the array from index 1 to 3 (excluding)

    print_slice(slice); // prints "2 3"
}
