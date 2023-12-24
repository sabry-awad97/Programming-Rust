fn main() {
    let mut numbers = [1, 2, 3, 4, 5];

    // Creating a slice of the first three elements
    let slice1 = &numbers[0..3];
    println!("Slice 1: {:?}", slice1);

    // Creating a slice of the last two elements
    let slice2 = &numbers[3..];
    println!("Slice 2: {:?}", slice2);

    // Creating a mutable slice to modify elements
    let mutable_slice = &mut numbers[1..4];
    for num in mutable_slice.iter_mut() {
        *num *= 2;
    }

    println!("Modified numbers: {:?}", numbers);
}
