fn main() {
    let c: char = 'A';
    assert!(c.is_alphabetic());
    assert!(c.is_uppercase());

    let d: char = '😄';
    assert!(!d.is_alphabetic());

    let e: char = 'a';
    let f: char = e.to_uppercase().next().unwrap();
    assert_eq!(f, 'A');
}
