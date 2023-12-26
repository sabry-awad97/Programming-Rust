fn main() {
    let data: &[i32] = &[1, 2, 3, 4, 5]; // Slice of integers

    let fat_pointer: *const [i32] = data as *const [i32]; // Creating a fat pointer

    // Not possible: let metadata = fat_pointer.len(); // Fat pointers don't have direct access to metadata

    // Using associated methods to access metadata
    let len = data.len(); // Accessing the length of the slice

    println!("Length: {}", len);
}
