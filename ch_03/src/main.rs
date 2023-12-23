fn main() {
    let mut num = 42;

    // Creating unsafe mutable raw pointer
    let unsafe_ptr: *mut i32 = &mut num;

    // Dereferencing the raw pointer within unsafe block
    unsafe {
        *unsafe_ptr = 10;
    }

    println!("Modified value: {}", num);
}
