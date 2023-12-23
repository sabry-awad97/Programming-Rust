fn main() {
    let c: char = 'a';
    let i: u32 = c as u32;
    let d: char = char::from_u32(i).unwrap();
    assert_eq!(c, d);
}
