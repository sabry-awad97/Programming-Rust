use std::{sync::Arc, thread, time::Duration};

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            println!("Thread {}: {:?}", i, data);
        });
    }

    std::thread::sleep(Duration::from_secs(1));
}
