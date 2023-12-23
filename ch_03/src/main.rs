fn main() {
    let t1 = (1, 2, 3);
    let t2 = (4, 5, 6);

    assert!(t1 != t2);
    assert!(t1 < t2);
    assert!(t1 <= t2);
    assert!(t2 > t1);
    assert!(t2 >= t1);

    let t3 = (1, 2, 3);

    assert!(t1 == t3);
    assert!(t1 <= t3);
    assert!(t1 >= t3);

    let t4 = (1, 2, 4);

    assert!(t1 < t4);
    assert!(t1 <= t4);
    assert!(t4 > t1);
    assert!(t4 >= t1);
}
