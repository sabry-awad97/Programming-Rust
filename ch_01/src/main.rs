use std::thread;

fn main() {
    let data = [1, 2, 3];

    // Spawn a new thread
    let handle = thread::spawn(move || {
        // Thread performs some computation
        let sum: i32 = data.iter().sum();
        println!("Sum in thread: {}", sum);
    });

    // Main thread continues its own work
    // ...

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
