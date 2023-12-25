fn main() {
    let s1 = String::from("Hello"); // s1 owns the heap-allocated string "Hello"

    let s2 = s1; // Ownership of the value proper (metadata) moves to s2

    // s1 is no longer valid here because ownership was moved to s2
    // println!("{}", s1); // This line won't compile

    // s2 still owns the same heap-allocated string "Hello"
    println!("{}", s2); // This works fine
}
