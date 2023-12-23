fn print_string(s: &String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");

    print_string(&s);

    println!("{}", s);
}
