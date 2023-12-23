use std::mem;

fn main() {
    let mut t = (1, 2, 3);

    let old = mem::replace(&mut t, (4, 5, 6));
    assert_eq!(old, (1, 2, 3));
    assert_eq!(t, (4, 5, 6));
}
