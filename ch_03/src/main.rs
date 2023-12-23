use std::cmp::*;

fn main() {
    let t1 = (1, 2, 3);
    let t2 = (4, 5, 6);

    let ord = t1.cmp(&t2);
    assert_eq!(ord, Ordering::Less);

    let t3 = (1, 2, 4);

    let ord = t1.cmp(&t3);
    assert_eq!(ord, Ordering::Equal);
}
