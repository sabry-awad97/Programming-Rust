fn main() {
    let v = vec!["A".to_string(), "B".to_string()];
    for i in v {
        let j = i; // i is moved to j on each iteration
        println!("{}", i); // error: i has been moved
    }
}
