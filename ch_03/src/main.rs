use std::collections::HashSet;

fn main() {
    let mut s = HashSet::new();
    s.insert((1, 2));
    s.insert((3, 4));

    assert!(s.contains(&(1, 2)));
    assert!(s.contains(&(3, 4)));
}
