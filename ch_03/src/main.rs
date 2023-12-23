use std::collections::BTreeSet;

fn main() {
    let mut s = BTreeSet::new();
    s.insert((1, 2));
    s.insert((3, 4));

    for x in s.iter() {
        println!("{:?}", x);
    }
}
