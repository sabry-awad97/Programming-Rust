use std::fs::File;

fn main() {
    let result: Result<&str, i32> = Err(42);
    let mapped_err = result.map_err(|err| err * 2);

    let open_file = |path| {
        File::open(path).map_err(|err| {
            println!("Failed to open file {}: {}", path, err);
            err
        })
    };

    let file = open_file("nonexistent.txt");
}
