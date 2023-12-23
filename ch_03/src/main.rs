use std::collections::BTreeMap;

fn main() {
    let mut m = BTreeMap::new();
    m.insert((1, 2), "foo");
    m.insert((3, 4), "bar");

    for (k, v) in m.iter() {
        println!("{:?}: {}", k, v);
    }
}
