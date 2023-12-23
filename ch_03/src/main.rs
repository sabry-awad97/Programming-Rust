fn main() {
    let x = f32::INFINITY;
    assert!(x.is_infinite());

    let y = f32::NAN;
    assert!(y.is_nan());

    let z: f32 = 1.0;
    assert!(z.is_finite());
}
