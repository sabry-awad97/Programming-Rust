fn main() {
    let c: char = 'a';
    println!("Size of 'a': {} bytes", std::mem::size_of_val(&c)); // prints "Size of 'a': 4 bytes"
}
