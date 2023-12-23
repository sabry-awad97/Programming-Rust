fn main() {
    // Addition
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_add(y);
    let wrapping_result = x.wrapping_add(y);
    let saturating_result = x.saturating_add(y);
    let (overflowing_result, overflowed) = x.overflowing_add(y);

    // Subtraction
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_sub(y);
    let wrapping_result = x.wrapping_sub(y);
    let saturating_result = x.saturating_sub(y);
    let (overflowing_result, overflowed) = x.overflowing_sub(y);

    // Multiplication
    let x = 2u8;
    let y = 3u8;

    let checked_result = x.checked_mul(y);
    let wrapping_result = x.wrapping_mul(y);
    let saturating_result = x.saturating_mul(y);
    let (overflowing_result, overflowed) = x.overflowing_mul(y);

    assert_eq!(checked_result, Some(6));
    assert_eq!(wrapping_result, 6);
    assert_eq!(saturating_result, 6);
    assert_eq!(overflowing_result, 6);
    assert!(!overflowed);

    // Division
    let x = 10u8;
    let y = 3u8;

    let checked_result = x.checked_div(y);
    let wrapping_result = x.wrapping_div(y);

    // Remainder
    let x = 10u8;
    let y = 3u8;

    let checked_result = x.checked_rem(y);
    let wrapping_result = x.wrapping_rem(y);

    // Left shift
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_shl(y as u32);
    let wrapping_result = x.wrapping_shl(y as u32);
    let (overflowing_result, overflowed) = x.overflowing_shl(y as u32);

    // Right shift
    let x = 1u8;
    let y = 2u8;

    let checked_result = x.checked_shr(y as u32);
    let wrapping_result = x.wrapping_shr(y as u32);
    let (overflowing_result, overflowed) = x.overflowing_shr(y as u32);
}
