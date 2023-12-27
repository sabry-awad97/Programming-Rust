fn main() {
    let x: Result<u32, &str> = Ok(2);
    let y: Result<u32, &str> = Err("late error");
    assert_eq!(x.or(y), Ok(2));
}
