fn main() {
    let x = 10;
    let mut y = "hello";
    let r1 = &x;
    let r2 = &mut y;

    assert!(r1 != r2); // cannot compare references of different types
}
