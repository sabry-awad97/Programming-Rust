fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}, world!", s1); // error: s1 has been moved
}
