fn main() {
    let s = String::from("hello");
    let r = &s; // Borrow s with a reference
    let t = r; // t is also a reference to s
    drop(s); // s goes out of scope and is deallocated
    println!("{}", t); // Error: t is a dangling reference
}
