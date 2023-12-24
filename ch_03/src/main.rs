fn main() {
    let s = String::from_utf8(vec![
        72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33,
    ])
    .unwrap(); // String

    println!("{}", s);
}
