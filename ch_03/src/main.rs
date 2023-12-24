fn main() {
    let mut xs = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    xs.dedup_by(|&mut a, &mut b| a % 2 == b % 2); // xs is now [1, 2, 3, 4]
}
