use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert((1, 2), "foo");
    m.insert((3, 4), "bar");

    assert_eq!(m.get(&(1, 2)), Some(&"foo"));
    assert_eq!(m.get(&(3, 4)), Some(&"bar"));
}
