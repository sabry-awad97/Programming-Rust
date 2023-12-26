fn main() {}

fn foo(x: &i32) {
    // x is a reference to an i32 with an unknown lifetime
}

fn bar<'a>(x: &'a i32) {
    // x is a reference to an i32 with the lifetime 'a
}
