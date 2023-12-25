use std::sync::{Arc, Mutex};

fn main() {
    // Creating shared data wrapped in an Arc and a Mutex
    let shared_data = Arc::new(Mutex::new(0));

    // Cloning Arc to create multiple references for different threads
    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    // Spawn threads to concurrently modify the shared data
    let thread1 = std::thread::spawn(move || {
        let mut data = shared_data_clone1.lock().unwrap();
        *data += 1; // Modifying the shared data
    });

    let thread2 = std::thread::spawn(move || {
        let mut data = shared_data_clone2.lock().unwrap();
        *data += 2; // Modifying the shared data
    });

    // Waiting for threads to complete execution
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Accessing the modified shared data
    let final_data = shared_data.lock().unwrap();
    println!("Final value: {}", *final_data);

    // Get the current reference count
    let count = Arc::strong_count(&shared_data);
    println!("Reference count of shared_data: {}", count);
}
