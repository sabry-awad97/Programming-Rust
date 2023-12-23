fn divmod(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

fn main() {
    let (q, r) = divmod(10, 3);
    assert_eq!(q, 3);
    assert_eq!(r, 1);
}
