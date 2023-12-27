use std::ops::Deref;

struct MyString(String);

impl Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn print_string(s: &str) {
    println!("{}", s);
}

fn main() {
    let my_string = MyString("hello".to_string());

    // The MyString type implements Deref, so we can pass a &MyString to a function
    // that takes a &str. The compiler will automatically apply deref coercion to
    // convert the &MyString to a &String, which can then be converted to a &str.
    print_string(&my_string);
}
