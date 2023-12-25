use difflib::differ::Differ;

fn main() {
    let original = "Hello, world!".split(' ').collect::<Vec<_>>();
    let modified = "Hi, world!".split(' ').collect::<Vec<_>>();

    let differ = Differ::new();
    let diff = differ.compare(&original, &modified);

    // Print the generated diff
    for line in diff {
        println!("{}", line);
    }
}
