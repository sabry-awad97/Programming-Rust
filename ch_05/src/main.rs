fn main() {
    let mut x = 10;
    let r1 = &mut x;
    let r2 = r1;
    *r2 = 20; // x is now 20
}
