fn main() {
    let result: Result<i32, &str> = Ok(42);
    let and_result = result.and(Ok(100));
}
