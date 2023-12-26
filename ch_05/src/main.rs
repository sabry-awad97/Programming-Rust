fn foo<'a>(x: &'a i32) -> (&i32, &i32) {
    (x, x)
}

fn main() {}
