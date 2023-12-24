fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let mut iter = xs.iter();
    assert_eq!(iter.next(), Some(&1)); // iter is now at the second element
    assert_eq!(iter.next(), Some(&2)); // iter is now at the third element
    assert_eq!(iter.next(), Some(&3)); // iter is now at the fourth element
    assert_eq!(iter.next(), Some(&4)); // iter is now at the fifth element
    assert_eq!(iter.next(), Some(&5)); // iter is at the end
    assert_eq!(iter.next(), None); // iter is at the end
}
