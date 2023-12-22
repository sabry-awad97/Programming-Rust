fn main() {
    let mut data = vec![1, 2, 3]; // Vector data

    let reference_to_data = &data; // Reference to data

    // Attempting to modify 'data' while it's borrowed causes a compile-time error
    // data.push(4); // This line would cause a compile error

    println!("Data: {:?}", reference_to_data); // Prints: Data: [1, 2, 3]

    // 'reference_to_data' goes out of scope here
    // 'data' can be modified again safely
    data.push(4); // This is allowed after the reference goes out of scope

    println!("Modified Data: {:?}", data); // Prints: Modified Data: [1, 2, 3, 4]
}
