fn main() {
    let b = Box::new(5);

    // `b` is moved into `b2`
    let b2 = b;

    // `b` is no longer valid
    // println!("{}", b); // error: use of moved value

    println!("{}", b2);
}
