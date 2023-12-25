use memchr::memchr;

fn main() {
    let haystack = b"Hello, Rust!"; // Byte slice to search within

    // Find the position of the byte `,` within the byte slice
    if let Some(position) = memchr(b',', haystack) {
        println!("Found at position: {}", position);
    } else {
        println!("Byte not found");
    }
}
