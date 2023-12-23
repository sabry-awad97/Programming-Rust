fn main() {
    let c: char = 'a';
    let d: char = c.to_uppercase().next().unwrap();
    assert_eq!(d, 'A');

    let e: char = 'A';
    let f: char = e.to_lowercase().next().unwrap();
    assert_eq!(f, 'a');

    let g: char = 'h';
    let h: char = g.to_ascii_uppercase();
    assert_eq!(h, 'H');
}
