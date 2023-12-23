fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            (m, n) = (n, m)
        }
        m %= n;
    }
    n
}

fn main() {
    assert_eq!(gcd(14, 21), 7);
    assert_eq!(gcd(49, 14), 7);
    assert_eq!(gcd(1, 100), 1);
    assert_eq!(gcd(6, 9), 3);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 21), 7);
    assert_eq!(gcd(49, 14), 7);
    assert_eq!(gcd(1, 100), 1);
    assert_eq!(gcd(6, 9), 3);
}

#[test]
#[should_panic]
fn test_gcd_of_zero_number() {
    let n = 30;
    let m = 0;
    gcd(n, m);
}
