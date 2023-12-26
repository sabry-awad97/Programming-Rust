fn main() {
    let x = 10;
    let y = 20;
    let mut r = &x;
    let b = true;
    if b {
        r = &y;
    }
    assert!(*r == 10 || *r == 20);
}
