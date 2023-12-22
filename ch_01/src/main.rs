use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let data = Arc::clone(&shared_data);
            thread::spawn(move || {
                // Lock the mutex to access shared data
                let mut val = data.lock().unwrap();
                *val += 1;
            })
        })
        .collect();

    for handle in handles {
        // Wait for the spawned thread to finish
        handle.join().unwrap();
    }

    // Access the final value of shared data
    println!("Final value: {:?}", shared_data.lock().unwrap());
}
