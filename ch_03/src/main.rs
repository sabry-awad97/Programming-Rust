use std::ffi::{CString, CStr};
use std::os::raw::c_char;

fn main() {
    // Create a CString from a string literal
    let s = CString::new("Hello, world!").unwrap();
    // Get a raw pointer to the CString's data
    let p: *const c_char = s.as_ptr();
    // Create a CStr from the raw pointer
    let c_str = unsafe { CStr::from_ptr(p) };
    // Convert the CStr to a Rust string slice
    let rust_str = c_str.to_str().unwrap();

    println!("{}", rust_str);
    // rust_str is now "Hello, world!"
}
