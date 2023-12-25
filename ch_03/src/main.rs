use shellexpand::tilde;

fn main() {
    let path_with_var = "~/Documents/{$USER}/file.txt"; // Represents a string with environment variables

    // Expand and resolve environment variables in the string
    let expanded_path = tilde(&path_with_var).into_owned();

    println!("Expanded Path: {}", expanded_path);
}
