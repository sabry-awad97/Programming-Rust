use std::collections::VecDeque;

fn main() {
    let mut s = VecDeque::new();
    s.push_back((1, 2));
    s.push_back((3, 4));

    assert_eq!(s.pop_back(), Some((3, 4)));
    assert_eq!(s.pop_back(), Some((1, 2)));
    assert_eq!(s.pop_back(), None);
}
