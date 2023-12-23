fn main() {
    let c: char = 'a';
    let s1: String = c.escape_unicode().collect();
    assert_eq!(s1, "\\u{61}");

    let s2: String = c.escape_debug().collect();
    assert_eq!(s2, "a");

    let s3: String = c.escape_default().collect();
    assert_eq!(s3, "a");

    let d: char = '\u{1f600}';
    let s4: String = d.escape_unicode().collect();
    assert_eq!(s4, "\\u{1f600}");

    let s5: String = d.escape_debug().collect();
    assert_eq!(s5, "😀");

    let s6: String = d.escape_default().collect();
    assert_eq!(s6, "\\u{1f600}");

    let s7: String = "a".to_string();
    let f: char = s7.parse().unwrap();
    assert_eq!(f, 'a');
}
