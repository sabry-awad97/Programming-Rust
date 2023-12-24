fn main() {
    let s1 = "Hello, world!".to_string();
    let s2 = "world".to_string();
    if s1.contains(&s2) {
        println!("'{}' is a substring of '{}'", s2, s1);
    }
}
