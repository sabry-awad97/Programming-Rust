// A Rust program showcasing safety, concurrency, performance, and zero-overhead principles

// Safety: Demonstrating memory safety with ownership and borrowing
fn demonstrate_safety() {
    let mut data = vec![1, 2, 3];
    let reference_to_data = &mut data;

    reference_to_data.push(4);
    println!("Demonstrating memory safety: {:?}", data);
}

// Concurrency: Using threads for concurrent execution
fn demonstrate_concurrency() {
    use std::thread;

    let handle = thread::spawn(|| {
        println!("Concurrent execution using threads");
    });

    handle.join().unwrap();
}

// Performance: Leveraging Rust's optimization and predictable performance
fn demonstrate_performance() {
    let _arr = [1, 2, 3];

    // Demonstrating array bounds checking for predictable performance
    // Uncommenting the line below would cause a run-time panic (index out of bounds)
    // let value = arr[5];
}

// Zero-Overhead Principle: Emphasizing efficient abstractions without runtime overhead
fn zero_overhead_principle() {
    // Rust provides high-level abstractions without performance penalties
    let maybe_number: Option<i32> = Some(42);

    if let Some(value) = maybe_number {
        println!(
            "Zero-overhead principle: Handling Option type safely - Value: {}",
            value
        );
    } else {
        println!("Zero-overhead principle: Handling Option type safely - No value!");
    }
}

fn main() {
    println!("Rust Program Summary:");

    // Demonstrating key aspects of Rust
    demonstrate_safety();
    demonstrate_concurrency();
    demonstrate_performance();
    zero_overhead_principle();
}
