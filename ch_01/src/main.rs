use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel for sending integers
    let (sender, receiver) = mpsc::channel();

    // Spawn a new thread to send messages
    let sender_thread = thread::spawn(move || {
        for i in 1..=5 {
            sender.send(i).unwrap(); // Sending integers through the channel
        }
    });

    // Main thread receives and prints the messages
    for received in receiver {
        println!("Received: {}", received);
    }

    // Wait for the sender thread to finish
    sender_thread.join().unwrap();
}
