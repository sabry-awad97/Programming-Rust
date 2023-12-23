fn main() {
    let a: u8 = 255; // Maximum value for u8
    let b: u8 = 1;

    // Overflow occurs when adding 'a' and 'b' as their sum exceeds the maximum value for u8
    let result = a.wrapping_add(b); // Using wrapping_add to demonstrate overflow behavior

    println!("Overflow Example: {}", result);
}
