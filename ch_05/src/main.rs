use std::ptr;

fn compare_memory_addresses(x: &i32, y: &i32) -> bool {
    ptr::eq(x, y)
}

fn main() {
    let x = 10;
    let y = 20;

    assert!(!compare_memory_addresses(&x, &y)); // x and y are stored at different memory addresses
    let r = &x;
    assert!(compare_memory_addresses(r, &x)); // r and &x point to the same memory address
}
