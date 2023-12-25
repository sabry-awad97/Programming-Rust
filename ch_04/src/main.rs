fn main() {
    let original = String::from("Hello"); // String allocated on the heap
    let new_var = original; // Ownership of the String moves to 'new_var'

    // Trying to access 'original' here would result in a compile-time error
    // because the ownership has been moved to 'new_var'
    // println!("{}", original); // This line won't compile

    // 'new_var' still owns the String data and can be used
    println!("{}", new_var); // This works fine
}
