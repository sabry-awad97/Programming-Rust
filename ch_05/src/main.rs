fn main() {
    let s = String::from("hello");
    let r = &s; // Borrow s with a reference
    let t = r; // t is also a reference to s
    let u = &r; // u is also a reference to s
    println!("{:p} {:p} {:p}", r, t, u);
}
