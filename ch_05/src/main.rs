fn main() {
    let x = 10;
    {
        let r = &x;
        assert_eq!(*r, 10);
    }
    assert_eq!(x, 10);
}
