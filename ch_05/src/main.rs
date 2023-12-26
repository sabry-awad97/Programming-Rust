fn main() {
    let x = 10;
    let y = 10;
    let r1 = &x;
    let r2 = &y;
    println!("{:p} {:p}", r1, r2); // different memory addresses
    assert!(r1 == r2); // r1 and r2 point to the same value
}
