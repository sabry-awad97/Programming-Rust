fn main() {
    let x: f64 = 2.0;
    let y: f32 = 3.0;

    // Error! No implicit conversion
    // let z = x + y;

    // addition
    let sum = x + y as f64;

    // subtraction
    let difference = x - y as f64;

    // multiplication
    let product = x * y as f64;

    // division
    let quotient = x / y as f64;

    // remainder
    let remainder = x % y as f64;
}
