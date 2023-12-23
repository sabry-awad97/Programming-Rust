use std::f64::consts::{E, PI, SQRT_2};

fn main() {
    let x = 2f64;

    // Calculate the square root of x
    let x_sqrt = x.sqrt();

    // Calculate the sine of x
    let x_sin = x.sin();

    // Calculate the cosine of x
    let x_cos = x.cos();

    // Calculate the tangent of x
    let x_tan = x.tan();

    // Calculate the natural logarithm of x
    let x_ln = x.ln();

    // Calculate the base-10 logarithm of x
    let x_log10 = x.log10();

    // Calculate the base-2 logarithm of x
    let x_log2 = x.log2();

    // Calculate the power of E raised to the x
    let e_to_the_x = E.powf(x);

    // Calculate the absolute value of x
    let x_abs = x.abs();

    // Calculate the maximum of x and PI
    let max = x.max(PI);

    // Calculate the minimum of x and SQRT_2
    let min = x.min(SQRT_2);
}
