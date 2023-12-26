fn f(p: &'static i32) {
    println!("{}", p);
}

fn main() {
    let x = 10;
    let y: &'static i32 = unsafe { std::mem::transmute(&x) };
    f(y);
}
