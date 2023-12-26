fn dangle() -> &i32 {
    let data = 42;
    &data // Returning a reference to 'data'
          // 'data' goes out of scope here
}

fn main() {
    let reference_to_nothing = dangle(); // Uncommenting this line would cause a compile-time error
    // Attempting to create a reference to 'data' that no longer exists
}
